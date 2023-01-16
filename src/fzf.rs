use skim::prelude::*;

pub fn prompt_user_selection(items: Box<dyn Iterator<Item = Box<dyn SkimItem>>>) -> Option<String> {
    let skim_options = SkimOptionsBuilder::default()
        .multi(true)
        .header(Some("Pick a resource"))
        .case(CaseMatching::Ignore)
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in items.into_iter() {
        tx_item
            .send(item.into())
            .expect("Failed to send item to Skim");
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&skim_options, Some(rx_item))
        .map(|out| match out.final_key {
            Key::ESC | Key::Ctrl('c') => vec![],
            _ => {
                println!("{:?}", out.final_key);
                out.selected_items
            }
        })
        .unwrap_or_else(|| Vec::new());

    selected_items
        .first()
        .map(|item| item.output().into_owned())
}
