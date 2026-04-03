use chrono::{Datelike, Local, NaiveDate};
use select::{document::Document, predicate};

use crate::{
    rankings::Ranking,
    team::{Team, Teams},
};

pub fn ncbwa_weeks(html: &str) -> Vec<String> {
    let doc = Document::from(html);

    doc.find(predicate::Name("h3"))
        .map(|h| h.parent().unwrap().attr("href").unwrap())
        .map(|u| format!("https://www.sportswriters.net{u}"))
        .collect()
}

pub fn ncbwa_specific(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let table = doc.find(predicate::Name("tbody")).next().unwrap();

    let mut teams = vec![];
    let mut rows = table.find(predicate::Name("tr"));
    // let date_regex =
    //     regex::Regex::new(r"(\d{4}) NCBWA DIVISION I POLL \((\w+) (\d+)\)").unwrap();
    let date_text = rows.next().unwrap().text();
    let date = NaiveDate::parse_from_str(&date_text, "%Y NCBWA DIVISION I POLL (%B %e)").unwrap();
    rows.next();
    for row in rows {
        let mut tds = row.find(predicate::Name("td"));
        let rank = tds.next().unwrap();
        let Some(name) = tds.next() else {
            continue;
        };
        tds.next();
        let name = name.text();
        let id = team_ids.get_id(&name);
        // log(&format!("FN {name} / {id:?}"));

        teams.push(Team {
            name,
            rank: rank.text().strip_suffix(".").unwrap().parse().unwrap(),
            first_votes: 0,
            votes: 0,
            id: id.map(str::to_owned).unwrap_or_default(),
            record: tds.next().unwrap().text(),
        });
    }

    Ranking { start: date, teams }
}

pub fn usa_today(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let script = doc
        .find(predicate::Attr("id", "__NEXT_DATA__"))
        .next()
        .unwrap();

    let mut j = json::parse(&script.text()).unwrap();
    let fallback = &mut j["props"]["pageProps"]["fallback"];

    let date = NaiveDate::parse_from_str(
        fallback["pollDetails"]["pollDate"].as_str().unwrap(),
        "%Y-%m-%d",
    )
    .unwrap();

    let mut teams = vec![];
    for main in [true, false] {
        let group = if main {
            &mut fallback["pollDetails"]["teamRanks"]
        } else {
            &mut fallback["otherReceivingVotes"]
        };
        for entry in group.members_mut() {
            let name = entry["teamName"].take_string().unwrap();

            let wins = entry["wins"].as_u32().unwrap();
            let losses = entry["losses"].as_u32().unwrap();
            let record = if let Some(ties) = entry["ties"].as_u32()
                && ties > 0
            {
                format!("{wins}-{losses}-{ties}")
            } else {
                format!("{wins}-{losses}")
            };

            teams.push(Team {
                id: team_ids
                    .get_id(&name)
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name,
                rank: entry["rank"].as_u32().filter(|r| *r <= 25).unwrap_or(0),
                first_votes: entry["firstPlaceVotes"].as_u32().unwrap(),
                votes: entry["points"].as_u32().unwrap(),
                record,
            });
        }
    }

    Ranking { start: date, teams }
}

// fn title_case

pub fn baseball_america(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let date = NaiveDate::parse_from_str(
        doc.find(predicate::Name("time"))
            .next()
            .unwrap()
            .text()
            .trim(),
        "%B %-d, %Y",
    )
    .unwrap();

    let tbody = doc.find(predicate::Name("tbody")).next().unwrap();
    let mut rows = tbody.find(predicate::Name("tr"));
    rows.next();

    let teams = rows
        .map(|row| {
            let mut parts = row.find(predicate::Name("td"));
            let rank = parts.next().unwrap().text().parse().unwrap();

            let a = parts.next().unwrap().first_child().unwrap();
            let mut href = a.attr("href").unwrap();
            href = href.strip_prefix("#").unwrap();
            let href = href.replace('-', " ");
            let name = a.text();
            let mut team_name = if name.to_lowercase().starts_with(&href) {
                name[..href.len()].into()
            } else {
                href.to_uppercase()
            };
            if team_name.as_str() == "TEXAS AM" {
                team_name = "Texas A&M".into();
            };

            parts.next();
            let rec = parts.next().unwrap().text();
            let rec = rec
                .split_once(" (")
                .map(|(overall, _conf)| overall)
                .unwrap_or(&rec);

            Team {
                id: team_ids
                    .get_id(&team_name)
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name: team_name,
                rank,
                first_votes: 0,
                votes: 0,
                record: rec.into(),
            }
        })
        .collect();

    Ranking { start: date, teams }
}

pub fn athletic_weeks(html: &str) -> Vec<String> {
    let doc = Document::from(html);

    doc.find(predicate::Name("a"))
        .filter_map(|a| a.attr("href"))
        .filter(|s| s.contains("college-baseball-ranking"))
        .map(|s| s.into())
        .collect()
}

pub fn athletic_specific(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let time_elem = doc.find(predicate::Name("time")).next().unwrap();
    let date = NaiveDate::parse_from_str(
        time_elem.attr("datetime").unwrap(),
        "%Y-%m-%dT%H:%M:%S%.3fZ",
    )
    .unwrap();

    let div = doc
        .find(predicate::Attr("id", "article-container-grid"))
        .next()
        .unwrap();
    let tbody = div.find(predicate::Name("tbody")).next().unwrap();

    let teams = tbody
        .find(predicate::Name("tr"))
        .map(|row| {
            let mut parts = row.find(predicate::Name("td"));

            let rank = parts.next().unwrap().text().parse().unwrap();
            let team_name = parts.next().unwrap().text();
            let _last_week = parts.next();
            let record = parts.next().unwrap().text();

            Team {
                id: team_ids
                    .get_id(team_name.trim())
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name: team_name,
                rank,
                first_votes: 0,
                votes: 0,
                record,
            }
        })
        .collect();

    Ranking { start: date, teams }
}

pub fn perfect_game_weeks(html: &str) -> Vec<String> {
    let doc = Document::from(html);

    let mut links = vec![];
    for div in doc.find(predicate::Class("card-header")) {
        let Some(a) = div.find(predicate::Name("a")).next() else {
            continue;
        };
        if !a.text().contains("Top 25") {
            continue;
        }
        let href = a.attr("href").unwrap();

        links.push(format!("https://www.perfectgame.org/Articles/{href}"));
    }

    links
}

pub fn perfect_game_specific(team_ids: &Teams, html: &str) -> Ranking {
    let doc = Document::from(html);

    let year = Local::now().year();
    let meta = doc
        .find(predicate::Attr("property", "og:title"))
        .next()
        .unwrap();
    let datestring = format!(
        "{};{year}",
        meta.attr("content")
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .trim()
    );
    let date = NaiveDate::parse_from_str(&datestring, "%B %d;%Y").unwrap();

    let tbody = doc
        .find(predicate::Class("table-responsive"))
        .next()
        .unwrap();
    let mut rows = tbody.find(predicate::Name("tr"));
    rows.next();
    let teams = rows
        .map(|row| {
            let mut parts = row.find(predicate::Name("td"));
            let rank = parts.next().unwrap().text().parse().unwrap();

            parts.next();
            let name = parts.next().unwrap().text();
            parts.next(); // last rank
            parts.next(); // record this week
            let record = parts.next().unwrap().text();

            Team {
                id: team_ids
                    .get_id(name.trim())
                    .map(str::to_owned)
                    .unwrap_or_default(),
                name,
                rank,
                first_votes: 0,
                votes: 0,
                record: record.trim_matches(['(', ')']).into(),
            }
        })
        .collect();

    Ranking { start: date, teams }
}
