mod childship;
mod mothership;
mod ship;

pub use ship::ShipPlugin;

pub mod prelude {
    pub use super::{childship::Childship, mothership::Mothership, ship::Ship};
}
