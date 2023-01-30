use std::{fs::File, io::Write};

use serde_bench::Canada;

const CANADA_JSON: &str = include_str!("../data/canada.json");

fn main() -> Result<(), std::io::Error> {
    let canada: Canada = serde_json::from_str(CANADA_JSON).unwrap();

    let json_string = serde_json::to_string(&canada).unwrap();
    let toml_string = toml::to_string(&canada).unwrap();
    let ron_string = ron::to_string(&canada).unwrap();
    let yaml_string = serde_yaml::to_string(&canada).unwrap();

    File::create("./data/canada.json")?.write_all(json_string.as_bytes())?;
    File::create("./data/canada.toml")?.write_all(toml_string.as_bytes())?;
    File::create("./data/canada.ron")?.write_all(ron_string.as_bytes())?;
    File::create("./data/canada.yaml")?.write_all(yaml_string.as_bytes())?;

    Ok(())
}
