use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use models::Action;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = db::JiraDatabase::new("./data/db.json".to_string());
    let db = Rc::new(db);
    let mut navigator = navigator::Navigator::new(Rc::clone(&db));

    // clearscreen::clear().unwrap();

    while let Some(current_page) = navigator.get_current_page() {
        current_page.draw_page().unwrap();
        let input = get_user_input();
        if let Ok(Some(action)) = current_page.handle_input(&input) {
            println!("action {:?}", action);
            navigator.handle_action(action).unwrap();
        }
    }
}
