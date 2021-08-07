use std::{fs, path::Path};
use itertools::{Itertools};
use regex::Regex;
use clap::{Arg, App};

mod date;
mod frontmatter;
mod csv_processor;


use date::Date;
use frontmatter::Frontmatter;

use crate::csv_processor::{to_csv, flatten_yaml};

extern crate yaml_rust;

fn main() -> Result<(), std::io::Error> {
    let option_matches = App::new("Frontmatter Processor")
        .version("0.1")
        .arg(Arg::new("path").default_value("."))
        .arg(Arg::new("filter").short('f').takes_value(true).multiple_occurrences(true).about("Filters only frontmatters with keys containing <filter>. Can be supplied multiple times and will be ORed together."))
        .arg(Arg::new("verbose").short('v').about("Print debug information during run"))
        .get_matches();

    let verbose = option_matches.is_present("verbose");
    let path_string = option_matches.value_of("path").expect("A path is required");

    let path = Path::new(path_string);

    // Collect Frontmatters
    let mut fms: Vec<Frontmatter> = fs::read_dir(path).unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            Frontmatter::load(&path)
        })
        .filter(|fm| {
            let contains_data = fm.contains_data();

            if !contains_data && verbose {
                println!("Skipping {}: no data", fm.path);
            }

            contains_data
        })
        .collect();

    // Filter Frontmatters
    let filter_strings = option_matches.values_of("filter");
    if let Some(filters) = filter_strings {
        let strings = filters.collect_vec();

        fms = fms
            .into_iter()
            .filter(|fm| {
                let kvo = flatten_yaml(None, &fm.yaml);
                match kvo {
                    Some(kvs) => {
                        let any_matches = kvs
                            .iter()
                            .any(|(k, _v)| strings.iter().any(|f| k.contains(f)));

                        if !any_matches && verbose {
                            println!("Skipping {}: filtered out", fm.path);
                        }

                        any_matches
                    },
                    None => false
                }
            })
            .collect();
    }

    // Error if no frontmatter found
    if fms.len() == 0 {
        let error = std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("no frontmatter found in directory {:?}", &path_string)
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
