use std::{collections::HashSet, time::Duration};
use termimad::{crossterm::style::Color, MadSkin};

mod todo_db;
mod todo_item;

use todo_db::TodoDB;

use self::todo_item::TodoItem;

fn get_todos() -> Vec<TodoItem> {
    let todos_db = TodoDB::load();

    todos_db.into_iter().collect()
}

fn print(todos: &[TodoItem]) {
    let mut skin = MadSkin::default_dark();
    // Title
    skin.set_fg(Color::Yellow);
    skin.print_text(r"# Todos");

    todos.iter().for_each(|todo| {
        skin.set_fg(Color::DarkGrey);
        skin.bold.set_fg(Color::Yellow);
        skin.print_text(format!(r"ID {} **{}**", todo.id, todo.title).as_str());
        if let Some(desc) = &todo.desc {
            skin.print_text(format!(r"* {}", desc).as_str());
        }
    });
}

pub fn list() {
    let todos = get_todos();
    print(&todos);
}

pub fn add(title: String, desc: Option<String>) {
    let mut todos_db = TodoDB::load();
    todos_db.create(title, desc);
    todos_db.save();
}

pub fn watch(secs_to_wait: u64) {
    let mut last_items: Option<Vec<TodoItem>> = None;

    loop {
        let items = get_todos();
        if last_items.is_none() {
            // Clear screen
            println!("\x1bc");
            print(&items);
            last_items = Some(items);
        } else if let Some(previous_items) = last_items.clone() {
            let prev_items_set: HashSet<TodoItem> = HashSet::from_iter(previous_items.clone());
            let items_set: HashSet<TodoItem> = HashSet::from_iter(items.clone());

            if prev_items_set != items_set {
                // Clear screen
                println!("\x1bc");
                print(&items);
                last_items = Some(items);
            }
        }

        std::thread::sleep(Duration::from_secs(secs_to_wait));
    }
}

pub fn edit(id: usize, new_title: Option<String>, new_desc: Option<String>) {
    let mut todos_db = TodoDB::load();
    match todos_db.update(id, new_title, new_desc) {
        Err(msg) => eprintln!("{}", msg),
        Ok(_) => println!("Successfully saved edits"),
    }
    todos_db.save();
}

pub fn delete(id: usize) {
    let mut todos_db = TodoDB::load();
    match todos_db.delete(id) {
        Err(msg) => eprintln!("{}", msg),
        Ok(_) => println!("Successfully deleted task {}", id),
    }
    todos_db.save();
}
