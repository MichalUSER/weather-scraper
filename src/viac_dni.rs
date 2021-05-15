use prettytable::{Cell, Row, Table};
use scraper::{Html, Selector};

pub fn get(document: &Html) {
    let vsetky_dni_selector = Selector::parse(".fcast-table-tabs > ul > li > a").unwrap();
    let den_selector = Selector::parse("span:nth-child(1)").unwrap();
    let datum_selector = Selector::parse("span:nth-child(2)").unwrap();
    let teplota_selector = Selector::parse("span:nth-child(3)").unwrap();

    let mut first_row: Vec<Cell> = Vec::new();
    let mut second_row: Vec<Cell> = Vec::new();

    for element in document.select(&vsetky_dni_selector) {
        let den = element.select(&den_selector).next().unwrap();
        let datum = element.select(&datum_selector).next().unwrap();
        let teplota = element.select(&teplota_selector).next().unwrap();
        first_row.push(Cell::new(
            &format!(
                "\x1b[0;34m{} {}\x1b[0m",
                den.text().collect::<String>().trim(),
                datum.text().collect::<String>().trim()
            )
            .to_string(),
        ));
        second_row.push(Cell::new(teplota.text().collect::<String>().trim()));
    }

    let mut table = Table::new();
    table.add_row(Row::new(first_row));
    table.add_row(Row::new(second_row));
    table.printstd();
}
