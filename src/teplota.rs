use scraper::{Html, Selector};

pub fn get(document: &Html) -> Option<()> {
    let temp_selector = Selector::parse(".temperature").unwrap();
    let temp_option = document.select(&temp_selector).next();

    if let Some(temp_text) = temp_option {
        let temp: String = temp_text.text().collect();
        println!("\x1b[0;34mTeplota:\x1b[0m {}", temp.trim());

        return Some(());
    }

    None
}
