use chrono::{Datelike, Days, Local, NaiveDate};
use select::{document::Document, predicate};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    rankings::Ranking,
    team::{Team, Teams},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn softball_america_weeks(html: &str) -> Vec<String> {
    let doc = Document::from(html);

    doc.find(predicate::Name("a"))
        .filter_map(|a| a.attr("href"))
        .filter(|s| s.contains("top-25"))
        .map(|s| format!("https://www.on3.com/{}", s))
        .collect()
}

pub fn softball_america_specifc(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let meta = doc
        .find(predicate::Attr("property", "article:published_time"))
        .next()
        .unwrap();
    let date = meta.attr("content").unwrap();
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%z").unwrap();

    let table = doc.find(predicate::Name("tbody")).next().unwrap();
    let mut rows = table.find(predicate::Name("tr"));
    rows.next();

    let teams = rows
        .map(|row| {
            let mut parts = row.find(predicate::Name("td"));

            let rank = parts.next().unwrap().text().parse().unwrap();
            let name = parts.next().unwrap().text();
            let record = parts.next().unwrap().text();

            Team {
                id: team_ids
                    .get_id(&name)
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name,
                rank,
                first_votes: 0,
                votes: 0,
                record,
            }
        })
        .collect();

    Ranking { start: date, teams }
}

pub fn usa_softball_specific(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let content = doc.find(predicate::Attr("id", "centcol")).next().unwrap();
    let text = content.find(predicate::Name("p")).next().unwrap().text();

    let date = NaiveDate::parse_from_str(
        &format!(
            "{}, {}",
            text.split_once("through ").unwrap().1.replace('.', ""),
            Local::now().year()
        ),
        "%B %-d, %Y",
    )
    .unwrap()
    .checked_add_days(Days::new(2))
    .unwrap(); // Move from Sunday to Tuesday?

    let table = doc.find(predicate::Name("tbody")).next().unwrap();
    let mut rows = table.find(predicate::Name("tr"));
    rows.next();

    let teams = rows
        .map(|row| {
            let mut parts = row.find(predicate::Name("td"));

            let rank = parts.next().unwrap().text().parse().unwrap();
            let name = parts.next().unwrap().text();
            let (name, first_votes) = match name.split_once(" (") {
                None => (name, 0),
                Some((n, f)) => (n.into(), f.strip_suffix(')').unwrap().parse().unwrap()),
            };
            let record = parts.next().unwrap().text();
            let votes = parts.next().unwrap().text().parse().unwrap();

            Team {
                id: team_ids
                    .get_id(&name)
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name,
                rank,
                first_votes,
                votes,
                record,
            }
        })
        .collect();

    Ranking { start: date, teams }
}

pub fn d1_softball_specific(team_ids: &Teams, json: &str) -> Ranking {
    let j = json::parse(json).unwrap();
    let post = j
        .members()
        .find(|p| {
            p["slug"]
                .as_str()
                .map(|s| s.contains("top-25"))
                .unwrap_or(false)
        })
        .unwrap();

    let date =
        NaiveDate::parse_from_str(post["date"].as_str().unwrap(), "%Y-%m-%dT%H:%M:%S").unwrap();

    let doc = Document::from(post["content"]["rendered"].as_str().unwrap());
    let tbody = doc.find(predicate::Name("tbody")).next().unwrap();
    let mut teams = vec![];
    for row in tbody.find(predicate::Name("tr")) {
        log(&format!("D1 SOFT ROW {}", row.html()));
        let mut parts = row.find(predicate::Name("td"));

        let Some(rank) = parts.next() else {
            continue;
        };
        let rank = rank.text().parse().unwrap();
        let name = parts.next().unwrap().text();
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
        });
    }
    // let teams =
    //     .map(|row| {

    //     })
    //     .collect();

    Ranking { start: date, teams }
}
