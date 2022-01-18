#![deny(clippy::pedantic, clippy::indexing_slicing)]
#![allow(clippy::unreadable_literal)]
#![cfg_attr(not(test), no_std)]

pub use element::{Element, IonRadius, State, Year};

mod element;
#[cfg(test)]
mod test;

include!(concat!(env!("OUT_DIR"), "/elements.rs"));

/// Return a list of elements in the periodic table
#[must_use]
pub fn periodic_table() -> &'static [&'static Element] {
    PERIODIC_TABLE
}
