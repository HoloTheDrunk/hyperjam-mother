mod baseship;
pub mod boids;
mod ships;

pub use baseship::ShipPlugin;

pub mod prelude {
    pub use super::{
        baseship::Ship,
        ships::{Childship, Mothership},
    };
}
