use crate::utils::prompt_user_selection;
use open;
use skim::prelude::*;
use std::borrow::Cow;

pub const MDN_OPTIONS_BYTES: &[u8] = include_bytes!("./../../data/mdn_urls.txt");
pub const MDN_BASE_URL: &str = "https://developer.mozilla.org/en-US/docs";

struct MdnDocItem {
    url_path: String,
}

impl SkimItem for MdnDocItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.url_path)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        // We have to collect and into_iter() right after because we can't reverse an unknown
        // length iterator
        #[allow(clippy::needless_collect)]
        let parts: Vec<String> = self
            .url_path
            .split('/')
            .map(|part| part.replace('_', " "))
            // Remove "Reference" since it just takes up a bunch of space
            .filter(|part| part != "Reference")
            .collect();

        // Each additional route is darker, so it fades from dark to light as you read left to
        // right
        let reversed_colored_segments: Vec<String> = parts
            .into_iter()
            .rev()
            .enumerate()
            .map(|(idx, part)| {
                let color_code = format!("\u{001b}[38;5;{}m", (255 - (idx * 4)));
                let colorcode_end = "\u{001b}[0m".to_string();
                format!("{color_code}{}{colorcode_end}", part)
            })
            .rev()
            .collect();

        let result = reversed_colored_segments.join(" ");

        AnsiString::parse(&result)
    }
}

pub fn search_and_open() {
    let mdn_urls = Box::leak(Box::new(String::from_utf8_lossy(MDN_OPTIONS_BYTES)));

    let items = mdn_urls
        .split('\n')
        .map(|url_path| MdnDocItem {
            url_path: url_path.to_string(),
        })
        .map(|item| Box::new(item) as Box<dyn SkimItem>);

    if let Some(selection) = prompt_user_selection("Open MDN Documentation for:", Box::new(items)) {
        let mdn_url = format!("{}{}", MDN_BASE_URL, selection);

        println!("Opening...");
        if let Err(e) = open::that(mdn_url) {
            panic!("Failed to open mdn documentation site. Reason: {e}");
        }
    }
}
