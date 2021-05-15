use scraper::{Html, Selector};

pub fn get(document: &Html) -> Option<()> {
    let temp_selector = Selector::parse(".temperature, .weather-image > svg > title").unwrap();
    let mut temp_option = document.select(&temp_selector);

    if let Some(temp_text) = temp_option.next() {
        let temp: String = temp_text.text().collect();
        let popis: String = temp_option.next().unwrap().text().collect();
        println!("\x1b[0;34mTeplota:\x1b[0m {} {}", temp.trim(), popis.trim());

        return Some(());
    }

    None
}
