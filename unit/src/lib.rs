//! The letter unit crate is a library to work with units and their conversion.
//! When writing letter we were facing a plethora of units used all over the place.
//! For example when dealing with fonts we usually use font-units - defined by the font-internal coordinate system.
//! When dealing with documents we usually want to have something metric like Millimeters.
//! But that is not entirely true.
//! For example when typesetting a document to be displayed in a digital environment it is probably useful to work with Pixels.
//! Therefore letter must work with any unit!

#[cfg(test)]
mod tests;

mod distance;
pub use distance::{Distance, DistanceUnit};

pub type UnitValue = f64;
