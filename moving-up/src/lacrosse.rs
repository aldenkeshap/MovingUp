use chrono::NaiveDate;
use regex::Regex;
use select::{document::Document, predicate};

use crate::{
    rankings::Ranking,
    team::{Team, Teams},
};

pub fn usila_weeks(html: &str) -> String {
    let doc = Document::from(html);

    let section = doc
        .find(predicate::Attr("id", "navigation"))
        .next()
        .unwrap();
    let script = section
        .find(predicate::Name("script"))
        .next()
        .unwrap()
        .text();
    let json = Regex::new(r"var component = (.+);\n")
        .unwrap()
        .captures(&script)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let mut j = json::parse(json).unwrap();

    j["data"]
        .members_mut()
        .find(|section| section["title"].as_str() == Some("Polls"))
        .unwrap()["items"]
        .members_mut()
        .find(|poll| poll["title"].as_str() == Some("Division I"))
        .unwrap()["url"]
        .take_string()
        .unwrap()
}

pub fn usila_specifc(team_ids: &Teams, html: &str) -> Ranking {
    // log(&format!("USILA S {team_ids:#?}"));
    let doc = Document::from(html);
    let date = NaiveDate::parse_from_str(
        &doc.find(predicate::Name("em")).next().unwrap().text(),
        "Updated on %B %e, %Y",
    )
    .unwrap();

    let table = doc.find(predicate::Name("tbody")).next().unwrap();

    let mut strongs = doc.find(predicate::Name("strong"));
    strongs.next();
    let footer = strongs.next().unwrap().next().unwrap().text();
    let received_votes = Regex::new(r"[:,](?:\sand)?\s([A-Za-z ]+) \((\d+)\)").unwrap();

    let teams = table
        .find(predicate::Name("tr"))
        .map(|row| {
            let mut parts = row.find(predicate::Name("td"));

            let name = parts.next().unwrap().text();
            let (name, first) = match name.split_once(" (") {
                Some((n, votes)) => (n.into(), votes.strip_suffix(')').unwrap().parse().unwrap()),
                None => (name, 0),
            };

            let rank = parts.next().unwrap().text().parse().unwrap();
            let votes = parts.next().unwrap().text().parse().unwrap();

            Team {
                id: team_ids
                    .get_id(name.trim())
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name,
                rank,
                first_votes: first,
                votes,
                record: "".into(),
            }
        })
        .chain(received_votes.captures_iter(&footer).map(|cap| {
            let (_, [name, votes]) = cap.extract();
            Team {
                id: team_ids
                    .get_id(name.trim())
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name: name.into(),
                rank: 0,
                first_votes: 0,
                votes: votes.parse().unwrap(),
                record: "".into(),
            }
        }))
        .collect();

    Ranking { start: date, teams }
}

pub fn usa_lacrosse_specifc(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);
    let mut strongs = doc.find(predicate::Name("strong"));

    strongs.next();
    let d = strongs.next().unwrap().next().unwrap().text();
    let d = d.trim();
    let date = NaiveDate::parse_from_str(d, "%B %e, %Y").unwrap();

    let tbody = doc.find(predicate::Name("tbody")).next().unwrap();
    let mut teams = vec![];
    for row in tbody.find(predicate::Name("tr")) {
        let mut parts = row.find(predicate::Name("td"));

        let rank = parts.next().unwrap().text().trim().parse().unwrap_or(0);
        let Some(name) = parts.next() else {
            continue;
        };
        let name = name.text().trim().to_owned();
        let record = parts.next().unwrap().text();

        teams.push(Team {
            id: team_ids
                .get_id(&name)
                .map(str::to_owned)
                .unwrap_or_default(),
            name,
            rank,
            first_votes: 0,
            votes: 0,
            record,
        })
    }

    Ranking { start: date, teams }
}
