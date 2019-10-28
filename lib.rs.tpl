// This file is auto generated. Modify build.rs instead of this file
pub use element::{Element, IonRadius, State, Year};

use lazy_static::lazy_static;

mod element;
#[cfg(test)]
mod test;

lazy_static! {
    /// The list of elements in the periodic table
    static ref PERIODIC_TABLE: Vec<Element> = vec![{{#each elements}}
        Element {
            atomic_number: {{atomic_number}},
            symbol: {{str symbol}},
            name: {{str name}},
            atomic_mass: {{str atomic_mass}},
            cpk_hex_color: {{str cpk_hex_color}},
            electronic_configuration: {{str electronic_configuration}},
            electronegativity: {{option_f32 electronegativity}},
            atomic_radius: {{option atomic_radius}},
            ion_radius: {{option_ion_radius ion_radius}},
            van_del_waals_radius: {{option van_del_waals_radius}},
            ionization_energy: {{option ionization_energy}},
            electron_affinity: {{option electron_affinity}},
            oxidation_states: {{vec oxidation_states}},
            standard_state: {{option_state standard_state}},
            bonding_type: {{str bonding_type}},
            melting_point: {{option melting_point}},
            boiling_point: {{option boiling_point}},
            density: {{option_f32 density}},
            group_block: {{str group_block}},
            year_discovered: {{year year_discovered}},
        },{{/each}}
    ];
}

/// Return a list of elements in the periodic table
pub fn periodic_table() -> &'static [Element] {
    &PERIODIC_TABLE
}
