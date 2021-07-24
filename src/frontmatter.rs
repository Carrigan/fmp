use std::{fs, path::PathBuf};
use super::Date;
use super::Regex;

#[derive(Debug)]
pub struct Frontmatter {
    pub path: String,
    pub yaml: yaml_rust::yaml::Yaml,
    pub date: Date
}

impl Frontmatter {
    pub fn load(path: &PathBuf) -> Option<Frontmatter> {
        let content = match path.extension().map(|p| p.to_str().unwrap()) {
            Some("yml") => fs::read_to_string(path).ok(),
            Some("md") => Frontmatter::extract_from_md(&fs::read_to_string(path).unwrap()),
            _ => None
        }?;

        let mut yamls = yaml_rust::YamlLoader::load_from_str(&content).ok()?;
        let yaml = yamls.pop()?;

        let filename_date = path
            .file_name()
            .map(|filename| filename.to_str())
            .flatten()
            .map(|filename| Date::from_filename(filename))
            .flatten();

        let date = match (Date::from_yaml(&yaml), filename_date) {
            (Some(date), _) => Some(date),
            (None, Some(date)) => Some(date),
            _ => None
        }?;

        let path_string = String::from(path.to_str()?);

        Some(Frontmatter { path: path_string, yaml, date })
    }

    fn extract_from_md(text: &str) -> Option<String> {
        let frontmatter_regex = Regex::new(r"^---\r?\n*([\s\S]*)\r?\n---\r?\n").unwrap();
        frontmatter_regex.captures(text)?.get(1).map(|cap| String::from(cap.as_str()))
    }
}

#[test]
fn test_extract_frontmatter() {
    let expected_extracted = "date: 210720\nrunning:\n  distance: 4.6\n  time: \"32:45\"";
    let test_markdown = fs::read_to_string("fixtures/test_frontmatter.md").unwrap();
    assert_eq!(Frontmatter::extract_from_md(&test_markdown).unwrap(), expected_extracted);
}
