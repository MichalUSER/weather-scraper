use prettytable::{Cell, Row, Table};
use scraper::{Html, Selector};

// Check if it rains etc

pub fn get(document: &Html) {
    let table_selector =
        Selector::parse("div.js-tab:nth-child(1) > div:nth-child(1) > .fcast-table-col").unwrap();

    let multiple_col = Selector::parse(
        "
        .fcast-table-head > .fs-l,
        .fcast-table-body > div:nth-child(-n+4) > span,
        .fcast-table-body > div:nth-child(7) > span,
        .fcast-table-body > div:nth-child(9) > span
    ",
    )
    .unwrap();

    let first_row: Vec<Cell> = vec![
        Cell::new(""),
        Cell::new("\x1b[0;34m2:00\x1b[0m"),
        Cell::new("\x1b[0;34m5:00\x1b[0m"),
        Cell::new("\x1b[0;34m8:00\x1b[0m"),
        Cell::new("\x1b[0;34m11:00\x1b[0m"),
        Cell::new("\x1b[0;34m14:00\x1b[0m"),
        Cell::new("\x1b[0;34m17:00\x1b[0m"),
        Cell::new("\x1b[0;34m20:00\x1b[0m"),
        Cell::new("\x1b[0;34m23:00\x1b[0m"),
    ];
    let mut second_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mTeplota\x1b[0m")];
    let mut third_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mPocitová teplota\x1b[0m")];
    let mut fourth_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mRiziko zrážok\x1b[0m")];
    let mut fifth_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mÚhrn zrážok\x1b[0m")];
    let mut sixth_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mOblačnosť\x1b[0m")];
    let mut seventh_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mRýchlosť vetra\x1b[0m")];
    let mut eighth_row: Vec<Cell> = vec![Cell::new("\x1b[0;34mRelatívna vlhkosť\x1b[0m")];

    for col in document.select(&table_selector) {
        for (index, info) in col.select(&multiple_col).enumerate() {
            // Make a function that adds text to a row
            // Make it not repetitive
            match index {
                0 => second_row.push(Cell::new(info.text().collect::<String>().as_str())),
                1 => third_row.push(Cell::new(info.text().collect::<String>().as_str())),
                2 => fourth_row.push(Cell::new(info.text().collect::<String>().as_str())),
                3 => fifth_row.push(Cell::new(info.text().collect::<String>().as_str())),
                4 => sixth_row.push(Cell::new(info.text().collect::<String>().as_str())),
                5 => seventh_row.push(Cell::new(info.text().collect::<String>().as_str())),
                6 => eighth_row.push(Cell::new(info.text().collect::<String>().as_str())),
                _ => (),
            }
        }
    }

    let mut table = Table::new();
    // Clean this
    table.add_row(Row::new(first_row));
    table.add_row(Row::new(second_row));
    table.add_row(Row::new(third_row));
    table.add_row(Row::new(fourth_row));
    table.add_row(Row::new(fifth_row));
    table.add_row(Row::new(sixth_row));
    table.add_row(Row::new(seventh_row));
    table.add_row(Row::new(eighth_row));
    table.printstd();
}
