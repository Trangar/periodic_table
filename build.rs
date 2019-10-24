use std::{fs::File, io::Write};

/// Generate the lib.rs file with an element list
/// TODO: Maybe this list can be a static array?
/// TODO: Maybe use lazy_static! ?
fn main() {
    let dest_path = "src/lib.rs";
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(
        b"
// This file is auto generated. Modify build.rs instead of this file

mod element;
pub use element::{Element, IonRadius, State, Year};
#[cfg(test)]
mod test;

/// Return a list of elements in the periodic table
pub fn periodic_table() -> Vec<Element> {
    let mut result = Vec::new();
",
    )
    .unwrap();

    let mut reader = csv::Reader::from_file("data.csv")
        .unwrap()
        .has_headers(true);
    let headers: Vec<String> = reader.headers().unwrap();
    let headers = headers
        .into_iter()
        .map(underscore_case)
        .collect::<Vec<String>>();
    for record in reader.decode() {
        let record = record.unwrap();
        let mut items: Vec<String> = record;
        for item in &mut items {
            *item = item.trim().to_string();
        }

        // pub atomic_number: u32,
        // pub symbol: &'static str,
        items[1] = format!("\"{}\"", items[1]);
        // pub name: &'static str,
        items[2] = format!("\"{}\"", items[2]);
        // pub atomic_mass: &'static str,
        items[3] = format!("\"{}\"", items[3]);
        // pub cpk_hex_color: &'static str,
        items[4] = format!("\"{}\"", items[4]);
        // pub electronic_configuration: &'static str,
        items[5] = format!("\"{}\"", items[5]);
        // pub electronegativity: Option<f32>,
        items[6] = if items[6].len() == 0 {
            String::from("None")
        } else {
            format!("Some({}_f32)", items[6])
        };
        // pub atomic_radius: Option<u32>,
        items[7] = if items[7].len() == 0 {
            String::from("None")
        } else {
            format!("Some({})", items[7])
        };
        // pub ion_radius: Option<IonRadius>,
        items[8] = if items[8].len() == 0 {
            String::from("None")
        } else {
            let mut parts = items[8].split(' ');
            let first = parts.next().unwrap().trim();
            let second = parts.next().unwrap().trim();
            format!("Some(IonRadius::new({}_f32, \"{}\"))", first, second)
        };
        // pub van_del_waals_radius: Option<u32>,
        items[9] = if items[9].len() == 0 {
            String::from("None")
        } else {
            format!("Some({})", items[9])
        };
        // pub ionization_energy: Option<u32>,
        items[10] = if items[10].len() == 0 {
            String::from("None")
        } else {
            format!("Some({})", items[10])
        };
        // pub electron_affinity: Option<i32>,
        items[11] = if items[11].len() == 0 {
            String::from("None")
        } else {
            format!("Some({})", items[11])
        };
        // pub oxidation_states: Vec<i32>,
        items[12] = format!("vec![{}]", items[12]);
        // pub standard_state: Option<State>,
        items[13] = if items[13].len() > 0 {
            format!("Some(State::{})", uppercase(&items[13]))
        } else {
            String::from("None")
        };
        // pub bonding_type: &'static str,
        items[14] = format!("\"{}\"", items[14]);
        // pub melting_point: Option<u32>,
        items[15] = if items[15].len() == 0 {
            String::from("None")
        } else {
            format!("Some({})", items[15])
        };
        // pub boiling_point: Option<u32>,
        items[16] = if items[16].len() == 0 {
            String::from("None")
        } else {
            format!("Some({})", items[16])
        };
        // pub density: Option<f32>,
        items[17] = if items[17].len() == 0 {
            String::from("None")
        } else {
            format!("Some({}_f32)", items[17])
        };
        // pub group_block: &'static str,
        items[18] = format!("\"{}\"", items[18]);
        // pub year_discovered: Year,
        items[19] = if items[19] == "Ancient" {
            String::from("Year::Ancient")
        } else {
            format!("Year::Known({})", items[19])
        };

        f.write_all(b"\tresult.push(Element {\n").unwrap();
        for i in 0..items.len() {
            let line = format!("\t\t{}: {},\n", headers[i], items[i]);
            f.write_all(line.as_bytes()).unwrap();
        }
        f.write_all(b"\t});\n").unwrap();
    }

    f.write_all(
        b"
    result
}",
    )
    .unwrap();
}

/// Return the string with every uppercase character turned to a lowercase and prefixed with an '_'
/// ``` rust
/// assert_eq!("some_text", underscore_case("SomeText"));
/// assert_eq!("camel_case", underscore_case("CamelCase"));
/// ```
fn underscore_case(name: String) -> String {
    let mut str = String::new();
    for (i, c) in name.trim().chars().enumerate() {
        if i == 0 {
            str.push_str(&c.to_lowercase().to_string());
        } else if c.is_uppercase() {
            str.push('_');
            str.push_str(&c.to_lowercase().to_string());
        } else {
            str.push(c);
        }
    }
    str
}

/// Return the string with the first character in uppercase
fn uppercase(string: &str) -> String {
    format!(
        "{}{}",
        string.chars().next().unwrap().to_uppercase(),
        &string[1..]
    )
}
