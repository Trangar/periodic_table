use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fs::File};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Clone, Copy)]
struct StrHelper {
    f: fn(&str) -> String,
}

impl StrHelper {
    fn boxed(f: fn(&str) -> String) -> Box<Self> {
        Box::new(StrHelper { f })
    }
}

impl HelperDef for StrHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).ok_or(RenderError::new("Missing variable"))?;
        let v = param.value().as_str().map(self.f);
        out.write(&v.unwrap_or_default())?;
        Ok(())
    }
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

fn init_templating() -> Handlebars {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("str", StrHelper::boxed(str_literal));
    handlebars.register_helper("option", StrHelper::boxed(option_literal));
    handlebars.register_helper("option_f32", StrHelper::boxed(option_f32_literal));
    handlebars.register_helper(
        "option_ion_radius",
        StrHelper::boxed(option_ion_radius_literal),
    );
    handlebars.register_helper("option_state", StrHelper::boxed(option_state_literal));
    handlebars.register_helper("vec", StrHelper::boxed(vec_literal));
    handlebars.register_helper("year", StrHelper::boxed(year_literal));

    handlebars
}

/// Generate the lib.rs file with an element list
fn main() -> Result<(), Box<dyn Error>> {
    let src_path = "lib.rs.tpl";
    let dest_path = "src/lib.rs";

    // create CSV reader
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path("data.csv")?;

    // get elements
    let elements = reader.deserialize().collect::<Result<Vec<Record>, _>>()?;

    // create template
    let template_name = "lib.rs";
    let mut handlebars = init_templating();
    handlebars.register_template_file(template_name, src_path)?;

    let mut data = BTreeMap::new();
    data.insert("elements".to_string(), elements);

    // render template to output file
    let mut out = File::create(&dest_path)?;
    handlebars.render_to_write(template_name, &data, &mut out)?;

    Ok(())
}
