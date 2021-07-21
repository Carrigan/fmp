use std::{env, fs, path::PathBuf};
use regex::Regex;

use yaml_rust::yaml;
extern crate yaml_rust;

#[derive(Debug)]
struct Frontmatter {
    yaml: yaml_rust::yaml::Yaml
}

impl Frontmatter {
    fn load(path: &PathBuf) -> Option<Frontmatter> {
        let content = match path.extension().map(|p| p.to_str().unwrap()) {
            Some("yml") => fs::read_to_string(path).ok(),
            Some("md") => Frontmatter::extract_from_md(&fs::read_to_string(path).unwrap()),
            _ => None
        }?;

        let mut yamls = yaml_rust::YamlLoader::load_from_str(&content).ok()?;
        let yaml = yamls.pop()?;

        Some(Frontmatter { yaml })
    }

    fn extract_from_md(text: &str) -> Option<String> {
        let frontmatter_regex = Regex::new(r"^---\r?\n*([\s\S]*)\r?\n---\r?\n").unwrap();
        frontmatter_regex.captures(text)?.get(1).map(|cap| String::from(cap.as_str()))
    }
}

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

#[test]
fn test_extract_frontmatter() {
    let expected_extracted = "date: 210720\nrunning:\n  distance: 4.6\n  time: \"32:45\"";
    let test_markdown = fs::read_to_string("fixtures/test_frontmatter.md").unwrap();
    assert_eq!(Frontmatter::extract_from_md(&test_markdown).unwrap(), expected_extracted);
}
