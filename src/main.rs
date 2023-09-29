use std::{fs::File, io::Write};

use serde_bench::Canada;

const CANADA_JSON: &str = include_str!("../data/canada.json");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let canada: Canada = serde_json::from_str(CANADA_JSON)?;

    let json_string = serde_json::to_string(&canada)?;
    let toml_string = toml::to_string(&canada)?;
    let ron_string = ron::to_string(&canada)?;
    let yaml_string = serde_yaml::to_string(&canada)?;

    let rmp_vec = rmp_serde::to_vec(&canada)?;
    let rmp_named_vec = rmp_serde::to_vec_named(&canada)?;
    let postcard_vec = postcard::to_stdvec(&canada)?;

    File::create("./data/canada.json")?.write_all(json_string.as_bytes())?;
    File::create("./data/canada.toml")?.write_all(toml_string.as_bytes())?;
    File::create("./data/canada.ron")?.write_all(ron_string.as_bytes())?;
    File::create("./data/canada.yaml")?.write_all(yaml_string.as_bytes())?;

    File::create("./data/canada.rmp")?.write_all(&rmp_vec)?;
    File::create("./data/canada.rmpnamed")?.write_all(&rmp_named_vec)?;
    File::create("./data/canada.postcard")?.write_all(&postcard_vec)?;

    Ok(())
}
