#![deny(clippy::pedantic, clippy::indexing_slicing)]
#![allow(clippy::unreadable_literal)]
#![cfg_attr(not(test), no_std)]

pub use element::{Element, IonRadius, State, Year};

mod element;
#[cfg(test)]
mod test;

include!(concat!(env!("OUT_DIR"), "/elements.rs"));

/// Return a list of elements in the periodic table.
///
/// Note that this is 0-indexed, so to get H/Hydrogen with atomic number 1, you actually need to fetch index `0`.
///
/// ```
/// let hydrogen = periodic_table::periodic_table()[0];
/// assert_eq!(hydrogen.symbol, "H");
/// assert_eq!(hydrogen.name, "Hydrogen");
/// ```
#[must_use]
pub fn periodic_table() -> &'static [&'static Element] {
    PERIODIC_TABLE
}
