use chrono::{DateTime, FixedOffset, Timelike};
use wasm_bindgen::prelude::*;

use json::parse;

use crate::{
    rankings::Ranking,
    sport::{Sport, SportEnum},
};

#[derive(Clone)]
struct Side {
    name: String,
    id: String,
    score: u32,
    rank: Option<u32>,
    record: String,
}
impl Side {
    fn name_rank(&self) -> String {
        match self.rank {
            // Some(99) => self.name.to_owned(),
            Some(r) => format!("{r} {}", self.name),
            None => self.name.to_owned(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone)]
enum Status {
    Past(u32),
    Ongoing(String),
    Suspended,
    Upcoming,
}
impl Status {
    fn from_name(name: &str, period: u32, clock: String) -> Option<Status> {
        Some(match name {
            "STATUS_SCHEDULED" => Status::Upcoming,
            "STATUS_IN_PROGRESS" | "STATUS_END_PERIOD" | "STATUS_HALFTIME" => {
                Status::Ongoing(clock)
            }
            "STATUS_FINAL" => Status::Past(period),
            "STATUS_SUSPENDED" => Status::Suspended,
            "STATUS_CANCELED" | "STATUS_POSTPONED" => return None,
            s => {
                log(s);
                Status::Upcoming
            }
        })
    }
}

// #[derive(Clone)]
// enum Clock {
//     Inning(String),
//     Time {
//         period: u32,
//         minutes: u32
//     },
// }

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Game {
    sport: Sport,
    pub id: String,
    date: chrono::NaiveDateTime,
    home: Side,
    away: Side,
    status: Status,
    broadcasts: Vec<String>,
}

#[wasm_bindgen]
impl Game {
    fn format_date(&self) -> String {
        if self.date.minute() == 0 {
            self.date.format("%a %-l%P")
        } else {
            self.date.format("%a %-l:%M%P")
        }
        .to_string()
    }
    pub fn format_broadcasts(&self) -> String {
        if self.broadcasts.is_empty() {
            "".into()
        } else {
            format!(" on {}", self.broadcasts.join("/"))
        }
    }

    pub fn show(&self, id: &str) -> String {
        let loc = if self.home.id == id { "vs" } else { "at" };
        let (this, other) = if self.home.id == id {
            (&self.home, &self.away)
        } else {
            (&self.away, &self.home)
        };
        match &self.status {
            Status::Upcoming => {
                format!("{loc} {}, {}", other.name_rank(), self.format_date())
            }
            Status::Ongoing(clock) => {
                format!(
                    "{loc} {} {}-{}, {clock}{}",
                    other.name_rank(),
                    this.score,
                    other.score,
                    self.format_broadcasts()
                )
            }
            Status::Suspended => {
                format!(
                    "{loc} {} {}-{}, suspended",
                    other.name_rank(),
                    this.score,
                    other.score,
                )
            }
            Status::Past(period) => {
                let res = if this.score > other.score { "W" } else { "L" };
                format!(
                    "{loc} {} {res}{} {}-{}",
                    other.name_rank(),
                    self.sport.extra(*period),
                    this.score,
                    other.score
                )
            }
        }
    }

    pub fn tooltip(&self) -> String {
        format!("{}{}", self.format_date(), self.format_broadcasts())
    }

    pub fn class(&self, id: &str) -> String {
        match self.status {
            Status::Upcoming => "future",
            Status::Suspended => "future",
            Status::Ongoing(_) => "current",
            Status::Past(_) => {
                let win = if self.away.id == id {
                    self.away.score > self.home.score
                } else {
                    self.home.score > self.away.score
                };
                if win { "win" } else { "loss" }
            }
        }
        .into()
    }

    pub fn record(&self, id: &str) -> String {
        if self.home.id == id {
            &self.home.record
        } else {
            &self.away.record
        }
        .into()
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct GamePerspectives {
    pub game: Game,
    pub perspectives: Vec<String>,
}
#[wasm_bindgen]
pub fn get_scores(
    sport: &Sport,
    ranked_teams: &Ranking,
    timezone_offset: i32,
    json: &str,
) -> Vec<GamePerspectives> {
    let timezone: FixedOffset = FixedOffset::west_opt(timezone_offset * 60).unwrap();

    let mut j = parse(json).unwrap();

    let mut game_perspectives = vec![];
    for event in j["events"].members_mut() {
        let Some(id) = event["id"].take_string() else {
            continue;
        };

        let competition = &mut event["competitions"][0];
        let period = competition["status"]["period"].as_u32().unwrap();
        let broadcasts = competition["geoBroadcasts"]
            .members_mut()
            .map(|j| j["media"]["shortName"].take_string().unwrap())
            .collect();

        let mut home = None;
        let mut away = None;
        for comp in competition["competitors"].members_mut() {
            let id = comp["id"].take_string().unwrap();
            let side = Side {
                name: comp["team"]["shortDisplayName"].take_string().unwrap(),
                rank: ranked_teams.get_ranking(&id),
                id,
                score: comp["score"].as_str().map(str::parse).unwrap().unwrap(),
                record: comp["records"][0]["summary"]
                    .take_string()
                    .unwrap_or("".into()),
            };
            match comp["homeAway"].as_str() {
                Some("home") => {
                    home = Some(side);
                }
                Some("away") => {
                    away = Some(side);
                }
                _ => {}
            }
        }

        let home = home.unwrap();
        let away = away.unwrap();

        let mut perspectives = vec![];
        for team in &ranked_teams.teams {
            if team.id == home.id || team.id == away.id {
                perspectives.push(team.id.clone());
            }
        }

        let clock = if matches!(sport.sport, SportEnum::Baseball | SportEnum::Softball) {
            event["status"]["type"]["shortDetail"]
                .take_string()
                .unwrap()
        } else if event["status"]["clock"].as_f64() == Some(0.) {
            let tied = home.score == away.score;
            match (period, sport.sport) {
                (_, _) if home.score == 0 && away.score == 0 => "about to start".into(),
                (1, SportEnum::MensBasketball) => "halftime".into(),
                (2, SportEnum::MensBasketball) if tied => "end of regulation".into(),
                (2, SportEnum::MensBasketball) => "final".into(),
                (3, SportEnum::MensBasketball) if tied => "end of OT".into(),
                (3, SportEnum::MensBasketball) => "final/OT".into(),
                (n, SportEnum::MensBasketball) if tied => format!("end of {}OT", n - 2),
                (n, SportEnum::MensBasketball) => format!("final/{}OT", n - 2),

                (1, _) => "end of Q1".into(),
                (2, _) => "halftime".into(),
                (3, _) => "end of Q3".into(),
                (4, _) if tied => "end of regulation".into(),
                (4, _) => "final".into(),
                (5, _) if tied => "end of OT".into(),
                (5, _) => "final/OT".into(),
                (n, _) if tied => format!("end of {}OT", n - 4),
                (n, _) => format!("final/{}OT", n - 4),
            }
        } else {
            let half = match (period, sport.sport) {
                (1, SportEnum::MensBasketball) => "1st".into(),
                (2, SportEnum::MensBasketball) => "2nd".into(),
                (3, SportEnum::MensBasketball) => "OT".into(),
                (n, SportEnum::MensBasketball) => format!("{}OT", n - 2),
                (n @ 1..=4, _) => format!("Q{}", n),
                (5, _) => "OT".into(),
                (n, _) => format!("{}OT", n - 4),
            };
            format!(
                "{} in {half}",
                event["status"]["displayClock"].take_string().unwrap()
            )
        };
        let Some(status) = Status::from_name(
            event["status"]["type"]["name"].as_str().unwrap(),
            period,
            clock,
        ) else {
            continue;
        };
        game_perspectives.push(GamePerspectives {
            game: Game {
                sport: *sport,
                id,
                home,
                away,
                status,
                date: DateTime::parse_from_rfc3339(
                    &event["date"].as_str().unwrap().replace('Z', ":00Z"),
                )
                .unwrap()
                .with_timezone(&timezone)
                .naive_local(),
                broadcasts,
            },
            perspectives,
        });
    }
    game_perspectives.sort_by_key(|gp| gp.game.date);
    game_perspectives
}
