use std::{fs::File, io::Write};

use serde_bench::Canada;

const CANADA_JSON: &str = include_str!("../data/canada.json");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let canada: Canada = serde_json::from_str(CANADA_JSON)?;

    serde_json::to_writer(File::create("./data/canada.json")?, &canada)?;
    File::create("./data/canada.toml")?.write_all(toml::to_string(&canada)?.as_bytes())?;
    File::create("./data/canada.ron")?.write_all(ron::to_string(&canada)?.as_bytes())?;
    serde_yaml::to_writer(File::create("./data/canada.yaml")?, &canada)?;

    File::create("./data/canada.rmpnamed")?.write_all(&rmp_serde::to_vec_named(&canada)?)?;
    File::create("./data/canada.rmp")?.write_all(&rmp_serde::to_vec(&canada)?)?;
    File::create("./data/canada.postcard")?.write_all(&postcard::to_stdvec(&canada)?)?;

    Ok(())
}
