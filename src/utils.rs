use std::io::Write;
use std::process::{Command, Stdio};

pub fn prompt_user_selection(header: &'static str, items: Vec<String>) -> Option<String> {
    let mut child = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(format!("--prompt={header}"))
        .spawn()
        .expect("Could not launch fzf");

    let options = items
        .into_iter()
        .fold(String::new(), |a, b| format!("{a}{b}\n"));
    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin
        .write_all(options.as_bytes())
        .expect("Failed to write to child stdin");

    let output = String::from_utf8(
        child
            .wait_with_output()
            .expect("Failed to wait for child output")
            .stdout,
    )
    .expect("failed to parse output as string");

    let trimmed = output.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
