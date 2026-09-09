use enum_macros::custom_discriminant;

use crate::rankings::Options;
use crate::rankings::OptionsList;
use crate::rankings::RankingOptions;
use crate::{rankings::Ranking, team::Teams};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
#[custom_discriminant(str)]
#[allow(clippy::enum_variant_names)]
pub enum Football {
    AP = "espn",
    Coaches = "coaches",
}
impl OptionsList for Football {
    const OPTIONS: &[RankingOptions] = &[
        RankingOptions::Football(Football::AP),
        RankingOptions::Football(Football::Coaches),
    ];
}

impl Options for Football {
    fn name(&self) -> &str {
        match self {
            Football::AP => "AP poll",
            Football::Coaches => "Coaches poll",
        }
    }
    fn get_url(&self) -> &str {
        match self {
            Football::AP | Football::Coaches => {
                "https://site.api.espn.com/apis/site/v2/sports/football/college-football/rankings"
            }
        }
    }
    fn get_weeks(&self, _teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>) {
        match self {
            Football::AP => (vec![], Some(Ranking::from_json_with_poll(text, Some("ap")))),
            Football::Coaches => (
                vec![],
                Some(Ranking::from_json_with_poll(text, Some("usa"))),
            ),
        }
    }
    fn get_specific(&self, _teams: &Teams, _text: &str) -> Ranking {
        unreachable!()
    }

    fn hide_points(&self) -> bool {
        false
    }
}
