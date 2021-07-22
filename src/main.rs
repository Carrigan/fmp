use std::{env, fs};
use regex::Regex;

mod date;
use date::Date;

mod frontmatter;
use frontmatter::Frontmatter;

extern crate yaml_rust;

fn main() {
    let mut fixtures = env::current_dir().unwrap();
    fixtures.push("fixtures");

    let ymls: Vec<Frontmatter> = fs::read_dir(fixtures).unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            Frontmatter::load(&path)
        })
        .collect();

    println!("{:?}", &ymls);
}
