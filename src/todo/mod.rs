use std::time::Duration;
use termimad::{crossterm::style::Color, MadSkin};

mod todo_db;
mod todo_item;

use todo_db::TodoDB;

pub fn list() {
    let mut skin = MadSkin::default_dark();
    let todos = TodoDB::load();

    // Title
    skin.set_fg(Color::Yellow);
    skin.print_text(r"# Todos");

    todos.into_iter().for_each(|todo| {
        skin.set_fg(Color::DarkGrey);
        skin.bold.set_fg(Color::Yellow);
        skin.print_text(format!(r"ID {} **{}**", todo.id, todo.title).as_str());
        if let Some(desc) = todo.desc {
            skin.print_text(format!(r"* {}", desc).as_str());
        }
    });
}

pub fn add(title: String, desc: Option<String>) {
    let mut todos = TodoDB::load();
    todos.create(title, desc);
    todos.save();
}

pub fn monitor() {
    loop {
        // Clear screen
        println!("\x1bc");

        list();

        std::thread::sleep(Duration::from_secs(5));
    }
}

pub fn edit(id: usize, new_title: Option<String>, new_desc: Option<String>) {
    let mut todos = TodoDB::load();
    match todos.update(id, new_title, new_desc) {
        Err(msg) => eprintln!("{}", msg),
        Ok(_) => println!("Successfully saved edits"),
    }
    todos.save();
}

pub fn delete(id: usize) {
    let mut todos = TodoDB::load();
    match todos.delete(id) {
        Err(msg) => eprintln!("{}", msg),
        Ok(_) => println!("Successfully deleted task {}", id),
    }
    todos.save();
}
