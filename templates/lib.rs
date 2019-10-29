// This file is auto generated. Modify build.rs instead of this file
pub use element::{Element, IonRadius, State, Year};

use lazy_static::lazy_static;

mod element;
#[cfg(test)]
mod test;

lazy_static! {
    /// The list of elements in the periodic table
    static ref PERIODIC_TABLE: Vec<Element> = vec![{% for e in elements %}
        Element {
            atomic_number: {{ e.atomic_number }},
            symbol: {{ e.symbol|str }},
            name: {{ e.name|str }},
            atomic_mass: {{ e.atomic_mass|str }},
            cpk_hex_color: {{ e.cpk_hex_color|str }},
            electronic_configuration: {{ e.electronic_configuration|str }},
            electronegativity: {{ e.electronegativity|option_f32 }},
            atomic_radius: {{ e.atomic_radius|option }},
            ion_radius: {{ e.ion_radius|option_ion_radius }},
            van_del_waals_radius: {{ e.van_del_waals_radius|option }},
            ionization_energy: {{ e.ionization_energy|option }},
            electron_affinity: {{ e.electron_affinity|option }},
            oxidation_states: {{ e.oxidation_states|vec }},
            standard_state: {{ e.standard_state|option_state }},
            bonding_type: {{ e.bonding_type|str }},
            melting_point: {{ e.melting_point|option }},
            boiling_point: {{ e.boiling_point|option }},
            density: {{ e.density|option_f32 }},
            group_block: {{ e.group_block|str }},
            year_discovered: {{ e.year_discovered|year }},
        },{% endfor %}
    ];
}

/// Return a list of elements in the periodic table
pub fn periodic_table() -> &'static [Element] {
    &PERIODIC_TABLE
}
