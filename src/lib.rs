// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_imports)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;

use chrono::Duration;
use chrono::naive::NaiveDate;

const FORMAT: &'static str = "%Y-%m-%d";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        base_url: url.to_base_url(),
        ctx: Context {
            user: None.
            token: None,
        },
        current_week_date: NaiveDate::from_ymd(2021, 02, 01),
        current_week_days: mock_week_days(current_week_date),
    }
}

fn mock_week_days(week_start: NaiveDate) -> Vec<Day> {
    let meals = BTreeMap::new();
    meals.insert(MealType::Lunch, Meal {
        option: MealType::Lunch,
        value: "hot dogs"
    });
    meals.insert(MealType.Supper, Meal {
        option: MealType::Supper,
        value: "chilli"
    });
    let mut days: Vec<Day> = Vec::new();
    for n in 0..7 {
        days.push(mock_day(week_start + Duration::dats(n)));
    }
    days
}

fn mock_day(title: NaiveDate) -> Day {
    let meals = BTreeMap::new();
    meals.insert(MealType::Lunch, Meal {
        option: MealType::Lunch,
        value: "hot dogs"
    });
    meals.insert(MealType.Supper, Meal {
        option: MealType::Supper,
        value: "chilli"
    });
    Day {
        title,
        meals,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    base_url: Url,
    ctx: Context,
    current_week_date: NaiveDate,
    current_week_days: Vec<Day>,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

struct User {
    username: String,
    email: String,
}

struct Day {
    title: NaiveDate,
    meals: BTreeMap<MealType, Meal>,
}

struct Meal {
    option: MealType,
    value: String,
}

enum MealType {
    Breakfast = 0,
    Lunch = 1,
    Supper = 2,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    UrlChanged(subs::UrlChanged),
    NextWeekTransition,
    PreviousWeekTransition,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {},
        Msg::NextWeekTransition => {
            let next_week = model.current_week_date + Duration::days(7);

        },
        Msg::PreviousWeekTransition => {
            let previous_week = model.current_week_date - Duration::days(7);
        },
    }
}

fn format_day(date: NaiveDate) -> String {
    format!("{}", date.format(FORMAT))
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["list-inline"],
        div![
            "Previous week <<",
            C!["counter"],
            button![ev(Ev::Click, |_| Msg::PreviousWeekTransition),],
        ],
        div![
            "Next week >>",
            C!["counter"],
            button![ev(Ev::Click, |_| Msg::NextWeekTransition),],
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
