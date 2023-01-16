use crate::fzf::prompt_user_selection;
use open;
use skim::prelude::*;
use std::borrow::Cow;

pub const MDN_OPTIONS_BYTES: &'static [u8] = include_bytes!("./../../data/mdn_urls.txt");
pub const MDN_BASE_URL: &str = "https://developer.mozilla.org/en-US/docs";

struct MdnDocItem {
    url_path: String,
}

impl SkimItem for MdnDocItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.url_path)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        let parts = self.url_path.split("/").map(|part| part.replace("_", " "));

        // Each additional route is darker, so it fades from dark to light as you read left to
        // right
        let mut result = String::with_capacity(64);
        for (idx, part) in parts.enumerate() {
            let color_code = format!("\u{001b}[38;5;{}m", ((idx * 3) + 235));
            let colorcode_end = format!("\u{001b}[0m");
            result = format!("{} {color_code}{}{colorcode_end}", result, part);
        }

        AnsiString::parse(&result)
    }
}

pub fn search_and_open() {
    let mdn_urls = Box::leak(Box::new(String::from_utf8_lossy(MDN_OPTIONS_BYTES)));

    let items = mdn_urls
        .split("\n")
        .map(|url_path| MdnDocItem {
            url_path: url_path.to_string(),
        })
        .map(|item| Box::new(item) as Box<dyn SkimItem>);

    if let Some(selection) = prompt_user_selection(Box::new(items)) {
        let mdn_url = format!("{}{}", MDN_BASE_URL, selection);

        println!("Opening...");
        if let Err(e) = open::that(mdn_url) {
            panic!("Failed to open mdn documentation site. Reason: {e}");
        }
    }
}

