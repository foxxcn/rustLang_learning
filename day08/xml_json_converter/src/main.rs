use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    person: Person
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err(anyhow!("Usage: {} <inputfile> <outputfile> <format>", args[0]));
    }
    let input_file = &args[1];
    let output_file = &args[2];
    let format = &args[3];

    match format.as_str() {
        "json_to_xml" => {
            let json_data = fs::read_to_string(input_file)?;
            let data: Data = serde_json::from_str(&json_data)?;
            let xml_data = serde_xml_rs::to_string(&data)?;
            fs::write(output_file, xml_data)?;
        },
        "xml_to_json" => {
            let mut xml_data = String::new();
            File::open(input_file)?.read_to_string(&mut xml_data)?;
            let data: Data = serde_xml_rs::from_str(&xml_data)?;
            let json_data = serde_json::to_string_pretty(&data)?;
            fs::write(output_file, json_data)?;
        },
        _ => return Err(anyhow!("Invalid format, use 'json_to_xml' or 'xml_to_json'")),
    }

    Ok(())
}