mod baseship;
pub mod boids;
mod ships;

pub use baseship::ShipPlugin;
pub use boids::BoidPlugin;

pub mod prelude {
    pub use super::{
        baseship::Ship,
        boids::BoidPlugin,
        ships::{Childship, Mothership},
    };
}
