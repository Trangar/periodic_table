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
    vec![
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
        items[1] = str_literal(&items[1]);
        // pub name: &'static str,
        items[2] = str_literal(&items[2]);
        // pub atomic_mass: &'static str,
        items[3] = str_literal(&items[3]);
        // pub cpk_hex_color: &'static str,
        items[4] = str_literal(&items[4]);
        // pub electronic_configuration: &'static str,
        items[5] = str_literal(&items[5]);
        // pub electronegativity: Option<f32>,
        items[6] = option_f32_literal(&items[6]);
        // pub atomic_radius: Option<u32>,
        items[7] = option_literal(&items[7]);
        // pub ion_radius: Option<IonRadius>,
        items[8] = option_ion_radius_literal(&items[8]);
        // pub van_del_waals_radius: Option<u32>,
        items[9] = option_literal(&items[9]);
        // pub ionization_energy: Option<u32>,
        items[10] = option_literal(&items[10]);
        // pub electron_affinity: Option<i32>,
        items[11] = option_literal(&items[11]);
        // pub oxidation_states: Vec<i32>,
        items[12] = vec_literal(&items[12]);
        // pub standard_state: Option<State>,
        items[13] = option_state_literal(&items[13]);
        // pub bonding_type: &'static str,
        items[14] = str_literal(&items[14]);
        // pub melting_point: Option<u32>,
        items[15] = option_literal(&items[15]);
        // pub boiling_point: Option<u32>,
        items[16] = option_literal(&items[16]);
        // pub density: Option<f32>,
        items[17] = option_f32_literal(&items[17]);
        // pub group_block: &'static str,
        items[18] = str_literal(&items[18]);
        // pub year_discovered: Year,
        items[19] = year_literal(&items[19]);

        f.write_all(b"\t\tElement {\n").unwrap();
        for i in 0..items.len() {
            let line = format!("\t\t\t{}: {},\n", headers[i], items[i]);
            f.write_all(line.as_bytes()).unwrap();
        }
        f.write_all(b"\t\t},\n").unwrap();
    }

    f.write_all(
        b"\t]
}\n",
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

/// Return the string surrounded with double quotes
fn str_literal(string: &str) -> String {
    format!("\"{}\"", string)
}

/// Return the literal wrapped in an option
fn option_literal(string: &str) -> String {
    if string.len() != 0 {
        format!("Some({})", string)
    } else {
        String::from("None")
    }
}

/// Return the literal wrapped in an option as an f32
fn option_f32_literal(string: &str) -> String {
    if string.len() != 0 {
        format!("Some({}_f32)", string)
    } else {
        String::from("None")
    }
}

/// Return the literal wrapped in an option as a IonRadius
fn option_ion_radius_literal(string: &str) -> String {
    if string.len() != 0 {
        let mut parts = string.split(' ');
        let first = parts.next().unwrap().trim();
        let second = parts.next().unwrap().trim();
        format!("Some(IonRadius::new({}_f32, \"{}\"))", first, second)
    } else {
        String::from("None")
    }
}

/// Return the literal wrapped in an option as a State
fn option_state_literal(string: &str) -> String {
    if string.len() != 0 {
        format!("Some(State::{})", uppercase(string))
    } else {
        String::from("None")
    }
}

/// Return the literal as a Year
fn year_literal(string: &str) -> String {
    if string == "Ancient" {
        String::from("Year::Ancient")
    } else {
        format!("Year::Known({})", string)
    }
}

/// Return the literal as a Vector
fn vec_literal(string: &str) -> String {
    format!("vec![{}]", string)
}
