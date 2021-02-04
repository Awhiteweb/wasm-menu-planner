// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(unused_imports)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;

use chrono::Duration;
use chrono::naive::NaiveDate;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        current_week_date: NaiveDate::from_ymd(2021, 02, 01)
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    current_week_date: NaiveDate
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    NextWeekTransition,
    PreviousWeekTransition,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NextWeekTransition => {
            model.current_week_date = model.current_week_date + Duration::days(7)
        },
        Msg::PreviousWeekTransition => {
            model.current_week_date = model.current_week_date - Duration::days(7)
        },
    }
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
            button![model, ev(Ev::Click, |_| Msg::PreviousWeekTransition),],
        ],
        div![
            "Next week >>",
            C!["counter"],
            button![model, ev(Ev::Click, |_| Msg::NextWeekTransition),],
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
