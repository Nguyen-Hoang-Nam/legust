use chrono::Datelike;
use clap::{App, Arg};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod agpl_30;
mod apache_20;
mod bsd_2_clause;
mod bsd_3_clause;
mod bsl_10;
mod cc0_10;
mod epl_20;
mod gpl_20;
mod gpl_30;
mod lgpl_21;
mod mit;
mod model;
mod mpl_20;
mod unlicense;

fn format_license(license: &model::License) -> String {
    let mut result: String = license.before.to_owned();
    let year: String = license.year.to_owned();
    let name: String = license.name.to_owned();
    let after: String = license.after.to_owned();

    result.push_str(&year);
    result.push_str(" ");
    result.push_str(&name);
    result.push_str(&after);

    return result;
}

fn main() -> std::io::Result<()> {
    let matches = App::new("legust")
        .version("1.1.0")
        .author("N.H Nam <nguyenhoangnam.dev@gmail.com>")
        .about("Add license to project")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .help("Change year in license")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Change username in license")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Change name of license file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The license to generate")
                .required(true)
                .index(1),
        )
        .get_matches();

    let license_name = matches.value_of("INPUT").unwrap_or("unlicense");

    let mut name = matches.value_of("name").unwrap_or("");
    let env_username;
    if name == "" {
        env_username = env::var("USERNAME").unwrap_or("".to_owned());
        name = &env_username;
    }

    let file_name = matches.value_of("output").unwrap_or("LICENSE");
    let file_name_pattern = Regex::new(r"^[\w\-. ]+$").unwrap();
    if !file_name_pattern.is_match(&file_name) {
        panic!("Invalid file name");
    }

    let mut year_raw = matches.value_of("year").unwrap_or("").to_string();
    let year_pattern = Regex::new(r"^\d{4}$").unwrap();
    if !year_pattern.is_match(&year_raw) {
        let current_date = chrono::Utc::now();
        year_raw = current_date.year().to_string();
    }

    let year: &str = &year_raw[..];

    let license_raw = match license_name {
        "agpl-3.0" => agpl_30::license(year, name),
        "apache_20" => apache_20::license(year, name),
        "bsd-2-clause" => bsd_2_clause::license(year, name),
        "bsd-3-clause" => bsd_3_clause::license(year, name),
        "bsl-1.0" => bsl_10::license(year, name),
        "cc0-1.0" => cc0_10::license(year, name),
        "epl-2.0" => epl_20::license(year, name),
        "gpl-2.0" => gpl_20::license(year, name),
        "gpl-3.0" => gpl_30::license(year, name),
        "lgpl-2.1" => lgpl_21::license(year, name),
        "mit" => mit::license(year, name),
        "mpl-2.0" => mpl_20::license(year, name),
        "unlicense" => unlicense::license(year, name),
        _ => panic!("License not found"),
    };

    let license = format_license(&license_raw);

    let mut file = File::create(file_name)?;
    file.write_all(license.as_bytes())?;
    Ok(())
}
