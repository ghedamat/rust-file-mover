extern crate toml;
use std::io::File;
use serialize::{Decodable, Decoder};

#[deriving(Decodable)]
pub struct TomlPaths {
    pub movie: String,
    pub music: String,
    pub var: String,
    pub trash: String,
}

#[deriving(Decodable)]
pub struct TomlManifest {
    pub paths: Option<Box<TomlPaths>>
}

pub fn read_config() -> Option<Box<TomlPaths>> {

    let contents = match File::open(&Path::new("config.toml")).read_to_string() {
        Ok(c) => c,
        Err(c) => fail!("Error: Cannot read config file")
    };
    let config = match toml::Parser::new(contents.as_slice()).parse() {
        Some(c) => c,
        None => fail!("Error: Failed to parse config file")
    };
    let mut d = toml::Decoder::new(toml::Table(config));
    let toml_manifest: Option<Box<TomlManifest>> = match Decodable::decode(&mut d) {
        Ok(t) => Some(t),
        Err(e) => fail!(format!("Error: Failed to parse config file, {}", e))
    };
    toml_manifest.unwrap().paths
}