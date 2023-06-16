use std::fs;

mod date_parser;
use date_parser::Date;

fn main() {
    // Searching for template file
    let template_filename = find_template();

    // Determining a name of a note
    let new_filename = filename_from_template(&template_filename);

    // Creating a file
    fs::copy(template_filename, &new_filename).expect("Couldn't copy the template file.");

    println!("{}.md created.", new_filename);
}

/// Creates filename from template.
/// day - current day.
/// month - current month.
/// year - current year.
fn filename_from_template(filename: &str) -> String {
    let mut iterator = filename.split('.');

    let extention = iterator.next_back().unwrap();

    // removing "template" at the beggining.
    let _ = iterator.next();

    let date = Date::new();

    let mut result = String::new();

    for param in iterator {
        let number = match param {
            "day" => date.day as u16,
            "month" => date.month as u16,
            "year" => date.year,
            _ => panic!("{} - unknown param.", param),
        };

        result += number.to_string().as_str();
        result += ".";
    }

    result += extention;

    result
}

/// Looking for the template file in the current derictory
fn find_template() -> String {
    for file in fs::read_dir("./").expect("Couldn't read current derictory") {
        if file.is_err() {
            continue;
        }

        let filename = file.unwrap().file_name().to_str().unwrap().to_string();
        if is_template(&filename) {
            return filename;
        }
    }
    panic!("Couldn't find a template file.");
}

/// Returns true if filename starts with "template" and ends with ".md"
fn is_template(filename: &String) -> bool {
    filename.starts_with("template") && filename.ends_with(".md")
}
