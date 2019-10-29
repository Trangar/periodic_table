#![deny(clippy::pedantic, clippy::indexing_slicing)]

use askama::Template;
use serde::Deserialize;
use std::{error::Error, fs::File, io::Write};

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Record {
    atomic_number: String,
    symbol: String,
    name: String,
    atomic_mass: String,
    cpk_hex_color: String,
    electronic_configuration: String,
    electronegativity: String,
    atomic_radius: String,
    ion_radius: String,
    van_del_waals_radius: String,
    ionization_energy: String,
    electron_affinity: String,
    oxidation_states: String,
    standard_state: String,
    bonding_type: String,
    melting_point: String,
    boiling_point: String,
    density: String,
    group_block: String,
    year_discovered: String,
}

#[derive(Template)]
#[template(path = "lib.rs", escape = "none")]
struct Data {
    elements: Vec<Record>,
}

mod filters {
    /// Return the string with the first character in uppercase
    fn uppercase(string: &str) -> String {
        format!(
            "{}{}",
            string.chars().next().unwrap().to_uppercase(),
            &string[1..]
        )
    }

    /// Return the string surrounded with double quotes
    pub fn str(string: &str) -> askama::Result<String> {
        Ok(format!("\"{}\"", string))
    }

    /// Return the literal wrapped in an option
    pub fn option(string: &str) -> askama::Result<String> {
        Ok(if string.len() != 0 {
            format!("Some({})", string)
        } else {
            String::from("None")
        })
    }

    /// Return the literal wrapped in an option as an f32
    pub fn option_f32(string: &str) -> askama::Result<String> {
        Ok(if string.len() != 0 {
            format!("Some({}_f32)", string)
        } else {
            String::from("None")
        })
    }

    /// Return the literal wrapped in an option as a IonRadius
    pub fn option_ion_radius(string: &str) -> askama::Result<String> {
        Ok(if string.len() != 0 {
            let mut parts = string.split(' ');
            let first = parts.next().unwrap().trim();
            let second = parts.next().unwrap().trim();
            format!(
                "Some(IonRadius {{ radius: {}_f32, variation: \"{}\" }})",
                first, second
            )
        } else {
            String::from("None")
        })
    }

    /// Return the literal wrapped in an option as a State
    pub fn option_state(string: &str) -> askama::Result<String> {
        Ok(if string.len() != 0 {
            format!("Some(State::{})", uppercase(string))
        } else {
            String::from("None")
        })
    }

    /// Return the literal as a Year
    pub fn year(string: &str) -> askama::Result<String> {
        Ok(if string == "Ancient" {
            String::from("Year::Ancient")
        } else {
            format!("Year::Known({})", string)
        })
    }

    /// Return the literal as a Vector
    pub fn slice(string: &str) -> askama::Result<String> {
        Ok(format!("&[{}]", string))
    }
}

/// Generate the lib.txt file with an element list
fn main() -> Result<(), Box<dyn Error>> {
    let dest_path = "src/lib.rs";

    // create CSV reader
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path("data.csv")?;

    // get elements
    let elements = reader.deserialize().collect::<Result<Vec<Record>, _>>()?;

    // render template
    let mut f = File::create(dest_path)?;
    let data = Data { elements };
    writeln!(f, "{}", data.render()?)?;

    Ok(())
}
