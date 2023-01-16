use std::time::Duration;

use jfs::Store;
use serde::{Deserialize, Serialize};

const DB_PATH: &'static str = ".rick/";
const TASKS_DB_ID: &'static str = "tasks";

#[derive(Debug, Serialize, Deserialize)]
struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
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
    let db = get_db();
    match db.get::<Tasks>(TASKS_DB_ID) {
        Err(e) => {
            eprintln!("{:?}", e);
        }
        Ok(Tasks { tasks }) => {
            println!("Tasks: {:#?}", tasks);
        }
    }
}

pub fn add(title: String, desc: Option<String>) {
    let db = get_db();
    let mut tasks: Tasks = db
        .get::<Tasks>(TASKS_DB_ID)
        .unwrap_or(Tasks { tasks: vec![] });
    tasks.tasks.push(Task {
        id: tasks.tasks.len(),
        title,
        desc,
    });

    db.save_with_id(&tasks, TASKS_DB_ID)
        .expect("Failed to save to db");
}

pub fn monitor() {
    loop {
        println!("\x1bc");

        let db = get_db();
        let tasks: Tasks = db
            .get::<Tasks>(TASKS_DB_ID)
            .unwrap_or(Tasks { tasks: vec![] });

        let blue_bg = "\u{001b}[48;5;18m";
        let dark_gray_bg = "\u{001b}[48;5;235m";
        let end = "\u{001b}[0m";
        tasks.tasks.into_iter().for_each(|task| {
            println!("ID: {} - {blue_bg}{}{end}", task.id, task.title);
            println!("{dark_gray_bg}{}{end}", task.desc.unwrap_or("".to_string()));
        });

        std::thread::sleep(Duration::from_secs(5));
    }
}
