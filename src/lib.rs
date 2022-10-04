#![allow(non_snake_case)]

mod flight;

pub mod prelude{
    pub use crate::flight::status;
    pub use crate::flight::departures;
}