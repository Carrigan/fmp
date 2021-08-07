use std::{collections::HashMap};
use itertools::Itertools;

use yaml_rust::Yaml;

use super::Frontmatter;

pub fn to_csv(fms: &Vec<Frontmatter>) -> String {
    let (flattened, keys) = process_frontmatters(&fms);

    let mut output = keys.iter().join(",");
    output += "\n";

    output += &flattened.iter().map(|d| {
        keys.iter().map(|key| {
            let blank = "".to_string();
            d.get(key).map(|v| v.clone()).unwrap_or(blank)
        }).join(",")
    }).join("\n");

    output
}

pub fn process_frontmatters(fms: &Vec<Frontmatter>) -> (Vec<HashMap<String, String>>, Vec<String>) {
    let flattened = fms
        .iter()
        .filter_map(|fm| {
            flatten_yaml(None, &fm.yaml)
                .map(|fl| {
                    let mut hash = kv_to_hash(fl);
                    hash.insert("date".to_string(), fm.date.to_string());
                    hash
                })
        })
        .collect();

    let keys = condense_keys(&flattened);

    (flattened, keys)
}

fn yaml_to_string(yml: &Yaml) -> String {
    match yml {
        Yaml::Real(val) => val.clone(),
        Yaml::Integer(val) => val.to_string(),
        Yaml::String(val) => val.clone(),
        Yaml::Boolean(val) => val.to_string(),
        Yaml::Array(val) => {
            val
                .iter()
                .map(|yml| yaml_to_string(yml))
                .fold(String::new(), |acc, current| {
                    acc + current.as_str()
                })
        },
        _ => String::from("todo")
    }
}

fn kv_to_hash(kv: Vec<(String, String)>) -> HashMap<String, String> {
    kv.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(k.clone(), v.clone());
        acc
    })
}

fn condense_keys(hashes: &Vec<HashMap<String, String>>) -> Vec<String> {
    let mapped = hashes
        .iter()
        .map(|hash| {
            hash.keys().map(|k| k.clone())
        });

    let mut sorted: Vec<String> = Iterator::flatten(mapped)
        .unique()
        .collect();

    sorted.sort();

    // Move `date` to front
    if let Some(date_index) = sorted.iter().position(|p| p == "date") {
        let date = sorted.remove(date_index);
        sorted.insert(0, date);
    }

    sorted
}

pub fn flatten_yaml(prefix: Option<String>, yaml: &Yaml) -> Option<Vec<(String, String)>> {
    let strings = yaml
        .as_hash()?
        .into_iter()
        .filter_map(|(k, v)| {
            match v {
                Yaml::Hash(_) => {
                    let mut new_prefix = String::new();

                    if let Some(prefix_str) = &prefix {
                        new_prefix += prefix_str.as_str();
                    }

                    new_prefix += k.as_str().unwrap();
                    new_prefix += "__";

                    flatten_yaml(Some(new_prefix), v)
                },
                _ => {
                    k
                        .as_str()
                        .map(|s| {
                            let qualified = match &prefix {
                                Some(p) => {
                                    let mut out = String::new();
                                    out += p.as_str();
                                    out += s;

                                    out
                                },
                                None => String::from(s)
                            };

                            vec!((qualified, yaml_to_string(v)))
                        })
                }
            }
        });

    let strings = Iterator::flatten(strings)
        .collect();

    Some(strings)
}

#[test]
fn test_extract_frontmatter() {
    use std::{path::PathBuf};

    let fm = Frontmatter::load(&PathBuf::from("fixtures/test_frontmatter.md")).unwrap();
    println!("{:?}", kv_to_hash(flatten_yaml(None, &fm.yaml).unwrap()))
}

#[test]
fn test_condense_keys() {
    let mut h1 = HashMap::new();
    h1.insert("k1".to_string(), "v1".to_string());
    h1.insert("k2".to_string(), "v2".to_string());

    let mut h2 = HashMap::new();
    h2.insert("k1".to_string(), "v1".to_string());
    h2.insert("k3".to_string(), "v3".to_string());

    let keys = condense_keys(&vec!(h1, h2));

    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"k1".to_string()));
    assert!(keys.contains(&"k2".to_string()));
    assert!(keys.contains(&"k3".to_string()));
}
