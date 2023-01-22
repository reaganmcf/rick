use std::iter::IntoIterator;
use std::{collections::HashMap, time::Duration};

use jfs::Store;
use serde::{Deserialize, Serialize};
use termimad::{crossterm::style::Color, MadSkin};

const DB_PATH: &'static str = ".rick/";
const TODOS_DB_ID: &'static str = "todos";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Todos {
    pub inner: HashMap<usize, Todo>,
    next_id: usize,
}

impl Todos {
    pub fn load() -> Self {
        let db = get_db();
        let todos: Self = db.get::<Self>(TODOS_DB_ID).unwrap_or(Self {
            inner: HashMap::default(),
            next_id: 0,
        });

        todos
    }

    pub fn add(&mut self, title: String, desc: Option<String>) {
        let new_id = self.next_id;
        self.next_id += 1;
        let new_todo = Todo {
            id: new_id,
            title,
            desc,
        };

        self.inner.insert(new_id, new_todo);
    }

    pub fn save(&mut self) {
        let db = get_db();
        db.save_with_id(self, TODOS_DB_ID)
            .expect("Failed to save to db");
    }

    pub fn edit(
        &mut self,
        id: usize,
        new_title: Option<String>,
        new_desc: Option<String>,
    ) -> Result<(), String> {
        match self.inner.get_mut(&id) {
            None => Err(format!("No matching todo with id {}", id)),
            Some(todo) => {
                if let Some(title) = new_title {
                    todo.title = title;
                }

                if new_desc.is_some() {
                    todo.desc = new_desc;
                }

                self.save();

                Ok(())
            }
        }
    }
}

impl IntoIterator for Todos {
    type Item = Todo;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // We don't care about the ID here
        let t: Vec<Self::Item> = self.inner.into_iter().map(|item| item.1).collect();
        t.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    desc: Option<String>,
}

fn get_db() -> Store {
    let mut cfg = jfs::Config::default();
    cfg.pretty = true;
    cfg.indent = 4;
    Store::new_with_cfg(DB_PATH, cfg).unwrap()
}

pub fn list() {
    let mut skin = MadSkin::default_dark();
    let todos = Todos::load();

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
    let mut todos = Todos::load();
    todos.add(title, desc);
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
    let mut todos = Todos::load();
    match todos.edit(id, new_title, new_desc) {
        Err(msg) => eprintln!("{}", msg),
        Ok(_) => println!("Successfully saved edits"),
    }
}
