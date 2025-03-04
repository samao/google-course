use std::ops::Deref;

use tracing::{debug, info};

pub fn trait_run() {
    info!("trait mode run");

    let msq = Meters(4).multiply(&Meters(3));
    
    debug!("result: {}", *msq);
}

#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

impl Deref for MetersSquared {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}