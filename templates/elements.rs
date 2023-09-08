/// The list of elements in the periodic table.
static PERIODIC_TABLE: &[&Element] = &[
    {% for e in elements %}
    &elements::{{ e.symbol|uppercase }},
    {% endfor %}
];

/// Elements that can be looked up from their symbol
pub mod elements {
    use super::{Element, IonRadius, State, Year};

    {% for e in elements %}
        /// {{ e.name }} ({{ e.symbol }}, #{{ e.atomic_number }})
        /// 
        /// - `atomic_number`: {{ e.atomic_number|doc }}
        /// - `symbol`: {{ e.symbol|doc }}
        /// - `name`: {{ e.name|doc }}
        /// - `atomic_mass`: {{ e.atomic_mass|doc }}
        /// - `cpk_hex_color`: {{ e.cpk_hex_color|doc }}
        /// - `electronic_configuration`: {{ e.electronic_configuration|doc }}
        /// - `electronegativity`: {{ e.electronegativity|doc }}
        /// - `atomic_radius`: {{ e.atomic_radius|doc }}
        /// - `ion_radius`: {{ e.ion_radius|doc }}
        /// - `van_del_waals_radius`: {{ e.van_del_waals_radius|doc }}
        /// - `ionization_energy`: {{ e.ionization_energy|doc }}
        /// - `electron_affinity`: {{ e.electron_affinity|doc }}
        /// - `oxidation_states`: {{ e.oxidation_states|doc }}
        /// - `standard_state`: {{ e.standard_state|doc }}
        /// - `bonding_type`: {{ e.bonding_type|doc }}
        /// - `melting_point`: {{ e.melting_point|doc }}
        /// - `boiling_point`: {{ e.boiling_point|doc }}
        /// - `density`: {{ e.density|doc }}
        /// - `group_block`: {{ e.group_block|doc }}
        /// - `year_discovered`: {{ e.year_discovered|doc }}
        pub static {{ e.symbol|uppercase }}: Element = Element {
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
            oxidation_states: {{ e.oxidation_states|slice }},
            standard_state: {{ e.standard_state|option_state }},
            bonding_type: {{ e.bonding_type|str }},
            melting_point: {{ e.melting_point|option }},
            boiling_point: {{ e.boiling_point|option }},
            density: {{ e.density|option_f32 }},
            group_block: {{ e.group_block|str }},
            year_discovered: {{ e.year_discovered|year }},
        };
    {% endfor %}
}

