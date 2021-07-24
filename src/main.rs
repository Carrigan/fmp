use std::{env, fs, path::Path};
use regex::Regex;
use clap::{AppSettings, Clap};

mod date;
mod frontmatter;
mod csv_processor;


use date::Date;
use frontmatter::Frontmatter;

use crate::csv_processor::to_csv;

extern crate yaml_rust;

#[derive(Clap)]
struct Opts {
    #[clap(default_value = ".")]
    path: String
}

fn main() -> Result<(), std::io::Error> {
    let opts: Opts = Opts::parse();
    let path = Path::new(&opts.path);

    // Collect Frontmatters
    let mut fms: Vec<Frontmatter> = fs::read_dir(path).unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            Frontmatter::load(&path)
        })
        .collect();

    // Error if no frontmatter found
    if fms.len() == 0 {
        let error = std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("no frontmatter found in directory {:?}", &opts.path)
        );

        return Err(error);
    }

    // Sort frontmatters by date
    fms.sort_by(|a, b| a.date.cmp(&b.date));

    // Process them to CSV
    let fmps = to_csv(&fms);

    print!("{}", fmps.as_str());

    Ok(())
}
