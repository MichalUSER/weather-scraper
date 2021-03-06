use scraper::Html;
use std::env;

mod dnes_info;
mod teraz;
mod viac_dni;
mod viac_info;

/*
"User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; rv:86.0) Gecko/20100101 Firefox/86.0"
*/

fn main() {
    let mut addition = "";
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: An argument is required");
        return;
    } else if args.len() == 3 {
        if args[2] == "z" {
            addition = "zajtra";
        }
    }

    if let Err(_) = get_weather(&args[1], addition) {
        println!("Can't get the weather");
    }
}

// Make output info more transparent
// Add option for what to show etc
// Just make it more readable
// Improve file names

pub fn get_weather(arg: &String, addition: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://pocasie.sme.sk/krajina/slovensko/{}/{}",
        arg, addition
    );
    let resp = attohttpc::get(url).send()?;
    let resp_text = resp.text()?;
    let document = Html::parse_document(resp_text.as_str());

    if let Some(_) = teraz::get(&document) {
        viac_info::get(&document);
        println!();
        if addition == "" {
            viac_dni::get(&document);
            println!();
        }
        dnes_info::get(&document);
    } else {
        println!("Can't get the weather");
    }

    Ok(())
}
