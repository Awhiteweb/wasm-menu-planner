// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        current_week_days: mock_week_days(),
        current_week_title: "2021-02-01"
    }
}

fn mock_week_days() -> Vec<Day> {
    let meals = BTreeMap::new();
    meals.insert(MealType::Lunch, Meal {
        option: MealType::Lunch,
        value: "hot dogs"
    });
    meals.insert(MealType.Supper, Meal {
        option: MealType::Supper,
        value: "chilli"
    });
    vec![
        Day {
            title: "2021-02-01",
            meals,
        }
    ]
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    current_week_days: Vec<Day>,
    current_week_title: String,
}

struct Day {
    title: String,
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
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    AddMeal(String title, MealType option, String value),
    CreateWeek(String title),
    NextWeekTransition,
    PreviousWeekTransition,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
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
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
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