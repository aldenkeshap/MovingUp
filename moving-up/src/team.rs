use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::sport;

// pub struct

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub rank: u32,
    pub first_votes: u32,
    pub votes: u32,
    pub record: String,
}

#[wasm_bindgen]
impl Team {
    #[wasm_bindgen]
    pub fn show_rank(&self) -> String {
        if self.first_votes > 0 {
            format!("{} ({})", self.rank, self.first_votes)
        } else if self.rank > 0 {
            format!("{}", self.rank)
        } else {
            "NR".into()
        }
    }

    pub fn fix_id(id: String) -> String {
        match id.as_str() {
            "529" => "1140".into(),
            _ => id,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Teams(HashMap<String, String>);
#[wasm_bindgen]
impl Teams {
    pub fn teams_url(sport: &sport::Sport) -> String {
        match *sport {
            sport::Sport {
                sport: sport::SportEnum::MensLacrosse,
            }
            | sport::Sport {
                sport: sport::SportEnum::WomensLacrosse,
            } => "https://site.api.espn.com/apis/site/v2/sports/basketball/mens-college-basketball/teams?groups=50&limit=500".into(),
            o => o.teams_url(),
        }
    }

    pub fn get_teams(json: &str) -> Teams {
        let mut j = json::parse(json).expect("parse");

        let mut teams = HashMap::new();
        for team in j["sports"][0]["leagues"][0]["teams"].members_mut() {
            let t = &mut team["team"];

            let Some(name) = t["location"].take_string() else {
                continue;
            };
            let id = t["id"].take_string().unwrap();
            if let Some(nickname) = t["nickname"].take_string() {
                teams.insert(nickname, id.clone());
            }
            teams.insert(name, id);
        }
        Teams(teams)
    }
}

impl Teams {
    pub fn get_id(&self, name: &str) -> Option<&str> {
        let name = match name {
            // found name -> ESPN name
            "North Carolina State" => "NC State",
            "Johns Hopkins" => return Some("118"),
            // "Penn" => "Pennsylvania",
            // "Pitt" => "Pittsburgh",
            "Boston" => "Boston University",
            "Loyola" => "Loyola Maryland",
            "U Mass Amherst" => "Massachusetts",
            "Southern California" => "USC",
            "Albany" => "UAlbany",
            "Miami (FL)" => "Miami",
            // "USF" => "South Florida",
            "Long Island University" | "Long Island" | "LIU" => return Some("2341"),
            n => n,
        };
        self.0.get(name).map(|n| n.as_str())
    }
}
