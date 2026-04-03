use chrono::{DateTime, Days, Local, NaiveDate};
use enum_dispatch::enum_dispatch;
use enum_macros::custom_discriminant;
use json::parse;
use wasm_bindgen::prelude::*;

use crate::{
    baseball, lacrosse, softball,
    sport::{Sport, SportEnum},
    team::{Team, Teams},
};

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Ranking {
    #[wasm_bindgen(skip)]
    pub start: chrono::NaiveDate,
    pub teams: Vec<Team>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl Ranking {
    fn from_json(json: &str) -> Ranking {
        Ranking::from_json_with_poll(json, None)
    }
    fn from_json_with_poll(json: &str, poll: Option<&str>) -> Ranking {
        let mut j = parse(json).expect("parse");

        log(&format!("FJWP 1 {}", json.len()));

        let mut teams = vec![];
        let mut date = None;
        for ranking in j["rankings"].members_mut() {
            log(&format!("FJWP {}", ranking["type"].pretty(2)));
            if poll.is_some() && ranking["type"].as_str() != poll {
                continue;
            }

            date = Some(
                DateTime::parse_from_rfc3339(
                    &ranking["date"].as_str().unwrap().replace('Z', ":00Z"),
                )
                .unwrap()
                .date_naive(),
            );

            for i in 0..2 {
                let group = match i {
                    0 => ranking["ranks"].members_mut(),
                    1 => ranking["others"].members_mut(),
                    _ => todo!(),
                };
                for t in group {
                    t.remove("logos");
                    teams.push(Team {
                        name: t["team"]["nickname"].take_string().expect("name"),
                        rank: t["current"].as_u32().expect("rank"),
                        first_votes: t["firstPlaceVotes"].as_u32().expect("first"),
                        votes: t["points"].as_f64().map(|f| f as u32).unwrap_or(123),
                        id: Team::fix_id(t["team"]["id"].take_string().unwrap()),
                        record: t["recordSummary"].take_string().unwrap(),
                    });
                }
            }
            break;
        }

        Ranking {
            teams,
            start: date.unwrap(),
        }
    }

    pub fn scoreboard_urls(&self, sport: &Sport) -> Vec<String> {
        (0..7)
            .map(|d| self.scoreboard_day(sport, self.start.checked_add_days(Days::new(d)).unwrap()))
            .collect()
    }
    fn scoreboard_day(&self, sport: &Sport, day: NaiveDate) -> String {
        format!(
            "https://site.api.espn.com/apis/site/v2/sports/{}/scoreboard?dates={}{}",
            sport.slug(),
            day.format("%Y%m%d"),
            if matches!(
                sport.sport,
                SportEnum::MensBasketball | SportEnum::WomensBasketball
            ) {
                "&groups=50"
            } else {
                ""
            }
        )
    }
    pub fn scoreboard_today(&self, sport: &Sport) -> String {
        let now = Local::now();
        let today = now.date_naive();

        self.scoreboard_day(sport, today)
    }

    pub fn get_ranking(&self, id: &str) -> Option<u32> {
        self.teams
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.rank)
            .filter(|r| *r > 0)
    }
}

#[wasm_bindgen]
pub struct RankingType {
    option: RankingOptions,
    weeks: Vec<String>,
    ranking: Option<Ranking>,
}

#[wasm_bindgen]
impl RankingType {
    fn new(option: RankingOptions) -> RankingType {
        RankingType {
            option,
            weeks: vec![],
            ranking: None,
        }
    }

    pub fn options(sport: &Sport) -> Vec<RankingType> {
        RankingOptions::options(*sport)
            .iter()
            .map(|o| RankingType::new(*o))
            .collect()
    }
    pub fn get_slug(&self) -> String {
        self.option.get_slug().into()
    }
    pub fn get_name(&self) -> String {
        self.option.name().into()
    }
    pub fn hide_points(&self) -> bool {
        self.option.hide_points()
    }

    pub fn ranking_ready(&self) -> bool {
        self.ranking.is_some()
    }
    pub fn get_ranking(&self) -> Ranking {
        self.ranking.as_ref().unwrap().clone()
    }
    pub fn get_url1(&self) -> String {
        self.option.get_url().into()
    }
    pub fn get_url2(&self) -> String {
        self.weeks[0].clone()
    }

    pub fn add_weeks(&mut self, teams: &Teams, res: &str) {
        let (weeks, ranking) = self.option.get_weeks(teams, res);
        self.weeks = weeks;
        self.ranking = ranking;
    }

    pub fn add_specific(&mut self, teams: &Teams, res: &str) {
        let ranking = self.option.get_specific(teams, res);
        self.ranking = Some(ranking);
    }
}

#[derive(Clone, Copy)]
#[enum_dispatch]
enum RankingOptions {
    MensBasketball,
    WomensBasketball,
    MensLacrosse,
    WomensLacrosse,
    Baseball,
    Softball,
}

impl RankingOptions {
    fn get_slug(&self) -> &str {
        match self {
            RankingOptions::MensBasketball(o) => o.custom_discriminant(),
            RankingOptions::WomensBasketball(o) => o.custom_discriminant(),
            RankingOptions::MensLacrosse(o) => o.custom_discriminant(),
            RankingOptions::WomensLacrosse(o) => o.custom_discriminant(),
            RankingOptions::Baseball(o) => o.custom_discriminant(),
            RankingOptions::Softball(o) => o.custom_discriminant(),
        }
    }
    fn options(sport: Sport) -> &'static [RankingOptions] {
        match sport.sport {
            SportEnum::MensBasketball => MensBasketball::OPTIONS,
            SportEnum::WomensBasketball => WomensBasketball::OPTIONS,
            SportEnum::MensLacrosse => MensLacrosse::OPTIONS,
            SportEnum::WomensLacrosse => WomensLacrosse::OPTIONS,
            SportEnum::Baseball => Baseball::OPTIONS,
            SportEnum::Softball => Softball::OPTIONS,
        }
    }
}

#[enum_dispatch(RankingOptions)]
trait Options {
    fn name(&self) -> &str;

    fn get_url(&self) -> &str;

    fn get_weeks(&self, teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>);

    fn get_specific(&self, teams: &Teams, text: &str) -> Ranking;

    fn hide_points(&self) -> bool;

    // fn options(&self) -> &[RankingOptions] {
    //     self::<OptionsList>::OPTIONS
    // }
}
trait OptionsList {
    const OPTIONS: &[RankingOptions];
}

trait BasketballOptions: Copy + Eq {
    const AP: Self;
    const USA: Self;

    const URL: &'static str;

    const POLL_OPTIONS: &[RankingOptions];
}

impl<T> OptionsList for T
where
    T: BasketballOptions,
{
    const OPTIONS: &[RankingOptions] = T::POLL_OPTIONS;
}

impl<T> Options for T
where
    T: BasketballOptions,
{
    // const OPTIONS: &[RankingOptions] = T::POLL_OPTIONS;

    fn name(&self) -> &str {
        match *self {
            ap if ap == T::AP => "AP Top 25",
            usa if usa == T::USA => "USA Today coaches poll",
            _ => unreachable!(),
        }
    }

    fn get_url(&self) -> &str {
        T::URL
    }

    fn get_weeks(&self, _teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>) {
        match *self {
            p if p == T::AP => (vec![], Some(Ranking::from_json_with_poll(text, Some("ap")))),
            p if p == T::USA => (
                vec![],
                Some(Ranking::from_json_with_poll(text, Some("usa"))),
            ),
            _ => unreachable!(),
        }
    }

    fn get_specific(&self, _teams: &Teams, _text: &str) -> Ranking {
        unreachable!()
    }

    fn hide_points(&self) -> bool {
        false
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[custom_discriminant(str)]
enum MensBasketball {
    APPoll = "espn",
    USAToday = "usa-today",
}
impl BasketballOptions for MensBasketball {
    const AP: Self = MensBasketball::APPoll;

    const USA: Self = MensBasketball::USAToday;

    const URL: &'static str =
        "https://site.api.espn.com/apis/site/v2/sports/basketball/mens-college-basketball/rankings";

    const POLL_OPTIONS: &[RankingOptions] = &[
        RankingOptions::MensBasketball(MensBasketball::APPoll),
        RankingOptions::MensBasketball(MensBasketball::USAToday),
    ];
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[custom_discriminant(str)]
enum WomensBasketball {
    APPoll = "espn",
    USAToday = "usa-today",
}
impl BasketballOptions for WomensBasketball {
    const AP: Self = WomensBasketball::APPoll;

    const USA: Self = WomensBasketball::USAToday;

    const URL: &'static str = "https://site.api.espn.com/apis/site/v2/sports/basketball/womens-college-basketball/rankings";

    const POLL_OPTIONS: &[RankingOptions] = &[
        RankingOptions::WomensBasketball(WomensBasketball::APPoll),
        RankingOptions::WomensBasketball(WomensBasketball::USAToday),
    ];
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
#[custom_discriminant(str)]
enum MensLacrosse {
    InsideLacrosse = "espn",
    USILA = "usila",
    USALacrosse = "usa-lacrosse",
}
impl OptionsList for MensLacrosse {
    const OPTIONS: &[RankingOptions] = &[
        RankingOptions::MensLacrosse(MensLacrosse::InsideLacrosse),
        RankingOptions::MensLacrosse(MensLacrosse::USILA),
        RankingOptions::MensLacrosse(MensLacrosse::USALacrosse),
    ];
}
impl Options for MensLacrosse {
    fn name(&self) -> &str {
        match self {
            MensLacrosse::InsideLacrosse => "Inside Lacrosse",
            MensLacrosse::USILA => "USILA coaches poll",
            MensLacrosse::USALacrosse => "USA Lacrosse top 20",
        }
    }

    fn get_url(&self) -> &str {
        match self {
            MensLacrosse::InsideLacrosse => {
                "https://site.api.espn.com/apis/site/v2/sports/lacrosse/mens-college-lacrosse/rankings"
            }
            MensLacrosse::USILA => "https://usila.org/archives.aspx?path=mlax",
            MensLacrosse::USALacrosse => {
                "https://www.usalacrosse.com/magazine/usa-lacrosse-division-i-mens-top-20"
            }
        }
    }

    fn get_weeks(&self, teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>) {
        match self {
            MensLacrosse::InsideLacrosse => (vec![], Some(Ranking::from_json(text))),
            MensLacrosse::USILA => (vec![lacrosse::usila_weeks(text)], None),
            MensLacrosse::USALacrosse => {
                (vec![], Some(lacrosse::usa_lacrosse_specifc(teams, text)))
            }
        }
    }

    fn get_specific(&self, teams: &Teams, text: &str) -> Ranking {
        match self {
            MensLacrosse::InsideLacrosse => unreachable!(),
            MensLacrosse::USILA => lacrosse::usila_specifc(teams, text),
            MensLacrosse::USALacrosse => unreachable!(),
        }
    }

    fn hide_points(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            MensLacrosse::USILA => false,
            _ => true,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
#[custom_discriminant(str)]
enum WomensLacrosse {
    InsideLacrosse = "espn",
    USALacrosse = "usa-lacrosse",
    // IWLCA = "iwlca",
}
impl OptionsList for WomensLacrosse {
    const OPTIONS: &[RankingOptions] = &[
        RankingOptions::WomensLacrosse(WomensLacrosse::InsideLacrosse),
        RankingOptions::WomensLacrosse(WomensLacrosse::USALacrosse),
    ];
}
impl Options for WomensLacrosse {
    fn name(&self) -> &str {
        match self {
            WomensLacrosse::InsideLacrosse => "Inside Lacrosse",
            WomensLacrosse::USALacrosse => "USA Lacrosse top 20",
            // WomensLacrosse::IWLCA => "ISLCA coaches poll",
        }
    }

    fn get_url(&self) -> &str {
        match self {
            WomensLacrosse::InsideLacrosse => {
                "https://site.api.espn.com/apis/site/v2/sports/lacrosse/womens-college-lacrosse/rankings"
            } // WomensLacrosse::IWLCA => todo!(),
            WomensLacrosse::USALacrosse => {
                "https://www.usalacrosse.com/magazine/usa-lacrosse-division-i-womens-top-20"
            }
        }
    }

    fn get_weeks(&self, teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>) {
        match self {
            WomensLacrosse::InsideLacrosse => (vec![], Some(Ranking::from_json(text))),
            // WomensLacrosse::IWLCA => todo!(),
            WomensLacrosse::USALacrosse => {
                (vec![], Some(lacrosse::usa_lacrosse_specifc(teams, text)))
            }
        }
    }

    fn get_specific(&self, _teams: &Teams, _text: &str) -> Ranking {
        todo!()
    }

    fn hide_points(&self) -> bool {
        true
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
#[custom_discriminant(str)]
enum Baseball {
    D1ESPN = "espn",
    NCBWA = "ncbwa",
    USAToday = "usa-today",
    #[allow(clippy::enum_variant_names)]
    BaseballAmerica = "baseball-america",
    Athletic = "athletic",
    PerfectGame = "perfect-game",
}
impl OptionsList for Baseball {
    const OPTIONS: &[RankingOptions] = &[
        RankingOptions::Baseball(Baseball::D1ESPN),
        RankingOptions::Baseball(Baseball::NCBWA),
        RankingOptions::Baseball(Baseball::USAToday),
        RankingOptions::Baseball(Baseball::BaseballAmerica),
        RankingOptions::Baseball(Baseball::Athletic),
        RankingOptions::Baseball(Baseball::PerfectGame),
    ];
}
impl Options for Baseball {
    fn name(&self) -> &str {
        match self {
            Baseball::NCBWA => "NCBWA writers poll",
            Baseball::D1ESPN => "D1Baseball",
            Baseball::USAToday => "USA Today coaches poll",
            Baseball::BaseballAmerica => "Baseball America",
            Baseball::Athletic => "The Athletic",
            Baseball::PerfectGame => "Perfect Game",
        }
    }

    fn get_url(&self) -> &str {
        use Baseball as Opts;
        match self {
            Opts::NCBWA => "https://www.sportswriters.net/ncbwa/news/tags/division-i-poll",
            Opts::D1ESPN => {
                "https://site.api.espn.com/apis/site/v2/sports/baseball/college-baseball/rankings"
            }
            Opts::USAToday => "https://sportsdata.usatoday.com/baseball/cbb/coaches-poll",
            Opts::BaseballAmerica => {
                "https://www.baseballamerica.com/stories/college-baseball-top-25-rankings/"
            }
            Opts::Athletic => "https://www.nytimes.com/athletic/tag/college-baseball/",
            Opts::PerfectGame => "https://www.perfectgame.org/Articles/Archive.aspx?Category=2",
        }
    }

    fn get_weeks(&self, teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>) {
        match self {
            Baseball::D1ESPN => (vec![], Some(Ranking::from_json(text))),
            Baseball::NCBWA => (baseball::ncbwa_weeks(text), None),
            Baseball::USAToday => (vec![], Some(baseball::usa_today(teams, text))),
            Baseball::BaseballAmerica => (vec![], Some(baseball::baseball_america(teams, text))),
            Baseball::Athletic => (baseball::athletic_weeks(text), None),
            Baseball::PerfectGame => (baseball::perfect_game_weeks(text), None),
        }
    }

    fn get_specific(&self, teams: &Teams, text: &str) -> Ranking {
        match self {
            Baseball::D1ESPN => unreachable!(),
            Baseball::NCBWA => baseball::ncbwa_specific(teams, text),
            Baseball::USAToday => unreachable!(),
            Baseball::BaseballAmerica => todo!(),
            Baseball::Athletic => baseball::athletic_specific(teams, text),
            Baseball::PerfectGame => baseball::perfect_game_specific(teams, text),
        }
    }

    fn hide_points(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Baseball::USAToday => false,
            _ => true,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
#[custom_discriminant(str)]
#[allow(clippy::enum_variant_names)]
enum Softball {
    USASoftball = "espn",
    SoftballAmerica = "softball-america",
    NFCA = "nfca",
    D1Softball = "d1-softball",
}
impl OptionsList for Softball {
    const OPTIONS: &[RankingOptions] = &[
        RankingOptions::Softball(Softball::USASoftball),
        RankingOptions::Softball(Softball::SoftballAmerica),
        RankingOptions::Softball(Softball::NFCA),
        RankingOptions::Softball(Softball::D1Softball),
    ];
}
impl Options for Softball {
    fn name(&self) -> &str {
        match self {
            Softball::USASoftball => "USA Softball",
            Softball::SoftballAmerica => "Softball America",
            Softball::NFCA => "NFCA coaches poll",
            Softball::D1Softball => "D1Softball",
        }
    }
    fn get_url(&self) -> &str {
        match self {
            Softball::USASoftball => {
                "https://site.api.espn.com/apis/site/v2/sports/baseball/college-softball/rankings"
            }
            Softball::SoftballAmerica => "https://www.on3.com/softball/news/",
            Softball::NFCA => {
                "https://nfca.org/component/com_nfca/Itemid,230/list,1/pdiv,div1/top25,1/"
            }
            Softball::D1Softball => {
                "https://d1softball.com/wp-json/wp/v2/posts?per_page=100&_fields=slug,date,content"
            }
        }
    }
    fn get_weeks(&self, teams: &Teams, text: &str) -> (Vec<String>, Option<Ranking>) {
        match self {
            Softball::USASoftball => (vec![], Some(Ranking::from_json(text))),
            Softball::NFCA => (vec![], Some(softball::usa_softball_specific(teams, text))),
            Softball::SoftballAmerica => (softball::softball_america_weeks(text), None),
            Softball::D1Softball => (vec![], Some(softball::d1_softball_specific(teams, text))),
        }
    }
    fn get_specific(&self, teams: &Teams, text: &str) -> Ranking {
        match self {
            Softball::SoftballAmerica => softball::softball_america_specifc(teams, text),
            _ => unreachable!(),
        }
    }

    fn hide_points(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Softball::NFCA => false,
            _ => true,
        }
    }
}
