use std::{env, fs};
use regex::Regex;

mod date;
mod frontmatter;
mod csv_processor;


use date::Date;
use frontmatter::Frontmatter;

use crate::csv_processor::to_csv;

extern crate yaml_rust;

fn main() {
    let mut fixtures = env::current_dir().unwrap();
    fixtures.push("fixtures");

    // Collect Frontmatters
    let mut fms: Vec<Frontmatter> = fs::read_dir(fixtures).unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            Frontmatter::load(&path)
        })
        .collect();

    // Sort frontmatters by date
    fms.sort_by(|a, b| a.date.cmp(&b.date));

    // Process them to CSV
    let fmps = to_csv(&fms);

    println!("{:?}", &fmps);
}
