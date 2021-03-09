// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_imports)]

#[macro_use] extern crate lazy_static;
extern crate regex;

use seed::{prelude::*, *};
use stdweb;

use web_sys::console;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::convert::TryInto;

use chrono::{Date, Datelike, NaiveDate, Local, Duration, Weekday, offset::TimeZone};
use regex::Regex;

const FORMAT: &'static str = "%Y-%m-%d";
const MEAL_FORMAT: &'static str = "%a %b %e";
const BREAKFAST: &str = "Breakfast";
const LUNCH: &str = "Lunch";
const SUPPER: &str = "Supper";

fn log(value: String) {
    console::log_1(&value.into());
}

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log(format!("{}", &url));
    orders.subscribe(Msg::UrlChanged);
    let today = get_today();
    let start_of_current_week = set_start_of_week(today);
    let start_of_week = get_display_week_date(Url::current(), &start_of_current_week);
    Model {
        base_url: url.to_hash_base_url().to_base_url(),
        ctx: Context {
            user: None,
            token: None,
        },
        current_week_date: start_of_current_week,
        display_week_date: start_of_week.clone(),
        display_week_days: mock_week_days(start_of_week),
        today: today,
    }
}

fn get_display_week_date(mut url: Url, current_week: &NaiveDate) -> NaiveDate {
    log(format!("{}", &url));
    match url.next_hash_path_part() {
        Some(date) => {
            match week_regex_tester(date) {
                Some(value) => {
                    NaiveDate::parse_from_str(value, FORMAT).unwrap()
                },
                None => current_week.clone()
            }
        },
        None => current_week.clone()
    }
}

/// function that calls javascript date.valueof object to get milliseconds from epoch
/// representstion of time, this is then converted to a std::time::Duration then converted to a
/// chrono::Duration and finally added to a chrono::NaiveDate set to epoch
/// this addition creates a new chrono::NaiveDate for today
fn get_today() -> NaiveDate {
    let ms: f64 = stdweb::web::Date::now();
    let epoch = chrono::NaiveDate::from_ymd(1970,1,1);
    let std_dur = std::time::Duration::from_millis(ms as u64);
    let chrono_dur = chrono::Duration::from_std(std_dur).unwrap();
    epoch + chrono_dur
}

fn set_start_of_week(day: impl Datelike) -> NaiveDate {
    let iso_week = day.iso_week().week();
    NaiveDate::from_isoywd(day.year(), iso_week, Weekday::Mon)
}

fn set_url_week(base_url: Url, week:NaiveDate) -> Url {
    base_url.add_hash_path_part(format_ymd(week))
}

fn mock_week_days(week_start: NaiveDate) -> Vec<Day> {
    let mut days: Vec<Day> = Vec::new();
    for n in 0..7 {
        days.push(mock_day(week_start + Duration::days(n)));
    }
    days
}

fn mock_day(title: NaiveDate) -> Day {
    let mut meals = BTreeMap::new();
    meals.insert(
        MealType::Lunch as i32,
        Meal {
            option: MealType::Lunch,
            value: "hot dogs".to_owned(),
        },
    );
    meals.insert(
        MealType::Supper as i32,
        Meal {
            option: MealType::Supper,
            value: "chilli".to_owned(),
        },
    );
    Day { title, meals }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    base_url: Url,
    ctx: Context,
    current_week_date: NaiveDate,
    display_week_date: NaiveDate,
    display_week_days: Vec<Day>,
    today: NaiveDate,
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
    meals: BTreeMap<i32, Meal>,
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

impl Model {
    fn previous_week_url(&self) -> Url {
        let previous_week = self.display_week_date - Duration::days(7);
        set_url_week(self.base_url.clone(), previous_week)
    }
    
    fn next_week_url(&self) -> Url {
        let next_week = self.display_week_date + Duration::days(7);
        set_url_week(self.base_url.clone(), next_week)
    }
}

/// tests if the input slice is a valid ymd date format
fn week_regex_tester<'a>(input: &'a str) -> Option<&'a str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    }
    match RE.is_match(input) {
        true => Some(input),
        false => None
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    UrlChanged(subs::UrlChanged),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log(format!("{}", &url));
            let week_start = get_display_week_date(url, &model.current_week_date);
            model.display_week_date = week_start.clone();
            model.display_week_days = mock_week_days(week_start);
        },
    }
}

// ------ ------
//    Helpers
// ------ ------

fn format_ymd(date: NaiveDate) -> String {
    format!("{}", date.format(FORMAT))
}

/// Takes the given date and outpus a string in the format %a %b %e
/// example: Monday Jan 1
fn format_meal_date(date: NaiveDate) -> String {
    format!("{}", date.format(MEAL_FORMAT))
}

fn meal_type_to_str(meal_type: &MealType) -> &str {
    match meal_type {
        MealType::Breakfast => BREAKFAST,
        MealType::Lunch     => LUNCH,
        MealType::Supper    => SUPPER, 
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(model.display_week_date, model.previous_week_url(), model.next_week_url()),
        view_main(&model.display_week_days, &model.today)
    ]
}

fn view_header(week: NaiveDate, previous_week_url: Url, next_week_url: Url) -> Node<Msg> {
    section![
        C!["menu-bar"],
        div![
            C!["item"],
            a![
                C!["btn draw-border"],
                attrs!{At::Href => previous_week_url},
                "Previous",
            ],
        ],
        div![
            C!["item", "title"],
            format_ymd(week)
        ],
        div![
            C!["item"],
            a![
                C!["btn draw-border"],
                attrs!{At::Href => next_week_url},
                "Next",
            ],
        ]
    ]
}

fn view_main(days: &Vec<Day>, today: &NaiveDate) -> Node<Msg> {
    section![
        C!["menu-body"],
        days.into_iter().map(|day| day_view(day, today)).collect::<Vec<Node<Msg>>>()
    ]
}

fn day_view(day: &Day, today: &NaiveDate) -> Node<Msg> {
    // create a new div for a day this will contain a title and list of meals
    div![
        C![
            "card",
            IF![&day.title == today => "today"]
        ],
        p![
            C!["card__name"],
            style!{
                St::MarginBottom => IF![&day.title == today => "20px"],
            },
            format_meal_date(day.title)
        ],
        div![
            C!["meal-list"],
            view_meals(&day.meals)
        ]
    ]
}

fn view_meals(meals: &BTreeMap<i32, Meal>) -> Vec<Node<Msg>> {
    meals.values().map(view_meal).collect()
}

fn view_meal(meal: &Meal) -> Node<Msg> {
    div![
        C!["meal"],
        h3![meal_type_to_str(&meal.option)],
        p![&meal.value]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
