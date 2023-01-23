use jfs::Store;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::IntoIterator;

use super::todo_item::TodoItem;

const DB_PATH: &'static str = ".rick/";
const TODOS_DB_ID: &'static str = "todos";

fn get_db() -> Store {
    let mut cfg = jfs::Config::default();
    cfg.pretty = true;
    cfg.indent = 4;
    Store::new_with_cfg(DB_PATH, cfg).unwrap()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoDB {
    pub inner: HashMap<usize, TodoItem>,
    next_id: usize,
}

impl TodoDB {
    pub fn load() -> Self {
        let db = get_db();
        let todos: Self = db.get::<Self>(TODOS_DB_ID).unwrap_or(Self {
            inner: HashMap::default(),
            next_id: 0,
        });

        todos
    }

    pub fn save(&mut self) {
        let db = get_db();
        db.save_with_id(self, TODOS_DB_ID)
            .expect("Failed to save to db");
    }

    pub fn create(&mut self, title: String, desc: Option<String>) {
        let new_id = self.next_id;
        self.next_id += 1;
        let new_todo = TodoItem {
            id: new_id,
            title,
            desc,
        };

        self.inner.insert(new_id, new_todo);
    }

    pub fn update(
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

                Ok(())
            }
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<TodoItem, String> {
        self.inner
            .remove(&id)
            .map_or(Err(format!("No matching todo with id {}", id)), |task| {
                Ok(task)
            })
    }
}

impl IntoIterator for TodoDB {
    type Item = TodoItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // We don't care about the ID here
        let t: Vec<Self::Item> = self.inner.into_iter().map(|item| item.1).collect();
        t.into_iter()
    }
}
