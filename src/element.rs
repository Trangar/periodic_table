/// Contains the information of a single element
#[derive(Debug)]
pub struct Element {
    pub atomic_number: u32,
    pub symbol: &'static str,
    pub name: &'static str,
    pub atomic_mass: &'static str,
    pub cpk_hex_color: &'static str,
    pub electronic_configuration: &'static str,
    pub electronegativity: Option<f32>,
    pub atomic_radius: Option<u32>,
    pub ion_radius: Option<IonRadius>,
    pub van_del_waals_radius: Option<u32>,
    pub ionization_energy: Option<u32>,
    pub electron_affinity: Option<i32>,
    pub oxidation_states: Vec<i32>,
    pub standard_state: Option<State>,
    pub bonding_type: &'static str,
    pub melting_point: Option<u32>,
    pub boiling_point: Option<u32>,
    pub density: Option<f32>,
    pub group_block: &'static str,
    pub year_discovered: Year,
}

/// The three possible states
#[derive(Debug)]
pub enum State {
    Solid,
    Liquid,
    Gas,
}

#[derive(Debug)]
pub struct IonRadius {
    pub radius: f32,
    pub variation: &'static str,
}

impl IonRadius {
    pub fn new(radius: f32, variation: &'static str) -> IonRadius {
        IonRadius { radius, variation }
    }
}

#[derive(Debug)]
pub enum Year {
    Ancient,
    Known(u16),
}
