use prettytable::{Cell, Row, Table};
use scraper::{Html, Selector};

pub fn get(document: &Html) {
    let info_selector = Selector::parse(".rows-wrapper > div > div").unwrap();
    let first_row_selector = Selector::parse(".data").unwrap();
    let second_row_selector = Selector::parse(".describe-label").unwrap();

    let mut first_row_vec: Vec<Cell> = Vec::new();
    let mut second_row_vec: Vec<Cell> = Vec::new();

    for element in document.select(&info_selector) {
        let first_row = element.select(&first_row_selector).next().unwrap();
        let second_row = element.select(&second_row_selector).next().unwrap();
        let name = second_row.inner_html();
        first_row_vec.push(Cell::new(format!("\x1b[0;34m{}\x1b[0m", name).as_str()));
        second_row_vec.push(Cell::new(first_row.text().collect::<String>().trim()));
    }

    let mut table = Table::new();
    table.add_row(Row::new(first_row_vec));
    table.add_row(Row::new(second_row_vec));
    table.printstd();
}
