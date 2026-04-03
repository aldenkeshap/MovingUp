use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SportEnum {
    MensBasketball,
    WomensBasketball,
    MensLacrosse,
    WomensLacrosse,
    Baseball,
    Softball,
}
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Sport {
    pub sport: SportEnum,
}

impl Sport {
    pub fn slug(&self) -> &str {
        match self.sport {
            SportEnum::MensBasketball => "basketball/mens-college-basketball",
            SportEnum::WomensBasketball => "basketball/womens-college-basketball",
            SportEnum::MensLacrosse => "lacrosse/mens-college-lacrosse",
            SportEnum::WomensLacrosse => "lacrosse/womens-college-lacrosse",
            SportEnum::Baseball => "baseball/college-baseball",
            SportEnum::Softball => "baseball/college-softball",
        }
    }
    pub fn extra(&self, period: u32) -> String {
        match (self.sport, period) {
            (SportEnum::Baseball, 9) | (SportEnum::Softball, 7) => String::new(),
            (SportEnum::Baseball, _) | (SportEnum::Softball, _) => format!("/{}", period),
            (SportEnum::MensBasketball, 2) => "".into(),
            (SportEnum::MensBasketball, 3) => "/OT".into(),
            (SportEnum::MensBasketball, p) => format!("/{}OT", p - 2),
            (
                SportEnum::WomensBasketball | SportEnum::MensLacrosse | SportEnum::WomensLacrosse,
                4,
            ) => "".into(),
            (
                SportEnum::WomensBasketball | SportEnum::MensLacrosse | SportEnum::WomensLacrosse,
                5,
            ) => "/OT".into(),
            (
                SportEnum::WomensBasketball | SportEnum::MensLacrosse | SportEnum::WomensLacrosse,
                p,
            ) => format!("/{}OT", p - 4),
        }
    }

    pub fn teams_url(&self) -> String {
        format!(
            "https://site.api.espn.com/apis/site/v2/sports/{}/teams?limit=500",
            self.slug()
        )
    }
}
#[wasm_bindgen]
impl Sport {
    #[allow(dead_code)]
    fn rankings_url(&self) -> String {
        format!(
            "https://site.api.espn.com/apis/site/v2/sports/{}/rankings",
            self.slug()
        )
    }

    #[allow(dead_code)]
    pub fn from_name(name: &str) -> Sport {
        Sport {
            sport: match name {
                "men-bball" => SportEnum::MensBasketball,
                "women-bball" => SportEnum::WomensBasketball,
                "men-lax" => SportEnum::MensLacrosse,
                "women-lax" => SportEnum::WomensLacrosse,
                "baseball" => SportEnum::Baseball,
                "softball" => SportEnum::Softball,
                _ => SportEnum::MensBasketball,
            },
        }
    }
}
