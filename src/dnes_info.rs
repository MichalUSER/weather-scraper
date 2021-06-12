use prettytable::{Cell, Row, Table};
use scraper::{ElementRef, Html, Selector};

fn to_cell(e: &ElementRef) -> Cell {
    Cell::new(e.text().collect::<String>().as_str())
}

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

    let first_row = vec![
        Cell::new(""),
        Cell::new("\x1b[0;34m2:00"),
        Cell::new("\x1b[0;34m5:00"),
        Cell::new("\x1b[0;34m8:00"),
        Cell::new("\x1b[0;34m11:00"),
        Cell::new("\x1b[0;34m14:00"),
        Cell::new("\x1b[0;34m17:00"),
        Cell::new("\x1b[0;34m20:00"),
        Cell::new("\x1b[0;34m23:00"),
    ];
    // Fix this
    let mut second_row = vec![Cell::new("\x1b[0;34mTeplota")];
    let mut third_row = vec![Cell::new("\x1b[0;34mPocitová teplota")];
    let mut fourth_row = vec![Cell::new("\x1b[0;34mRiziko zrážok")];
    let mut fifth_row = vec![Cell::new("\x1b[0;34mÚhrn zrážok")];
    let mut sixth_row = vec![Cell::new("\x1b[0;34mOblačnosť")];
    let mut seventh_row = vec![Cell::new("\x1b[0;34mRýchlosť vetra")];
    let mut eighth_row = vec![Cell::new("\x1b[0;34mRelatívna vlhkosť")];

    for col in document.select(&table_selector) {
        for (index, info) in col.select(&multiple_col).enumerate() {
            // Make a function that adds text to a row
            let info_cell = to_cell(&info);
            match index {
                0 => second_row.push(info_cell),
                1 => third_row.push(info_cell),
                2 => fourth_row.push(info_cell),
                3 => fifth_row.push(info_cell),
                4 => sixth_row.push(info_cell),
                5 => seventh_row.push(info_cell),
                6 => eighth_row.push(info_cell),
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
