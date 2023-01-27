use crate::utils::prompt_user_selection;
use open;

pub const MDN_OPTIONS_BYTES: &[u8] = include_bytes!("./../../data/mdn_urls.txt");
pub const MDN_BASE_URL: &str = "https://developer.mozilla.org/en-US/docs";

pub fn search_and_open() {
    let mdn_urls = Box::leak(Box::new(String::from_utf8_lossy(MDN_OPTIONS_BYTES)));

    let items = mdn_urls.split('\n').map(String::from).collect();
    //.map(|url_path| MdnDocItem {
    //    url_path: url_path.to_string(),
    //})
    //.map(|item| Box::new(item) as Box<dyn SkimItem>);

    if let Some(selection) = prompt_user_selection("Open MDN Documentation for:", items) {
        let mdn_url = format!("{}{}", MDN_BASE_URL, selection);

        println!("Opening...");
        if let Err(e) = open::that(mdn_url) {
            panic!("Failed to open mdn documentation site. Reason: {e}");
        }
    }
}
