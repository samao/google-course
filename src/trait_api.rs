use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use tracing::{debug, info};

pub fn trait_run() {
    info!("trait mode run");

    let msq = Meters(4).multiply(&Meters(3));

    debug!("result: {}", *msq);

    let mut l = LittleBox {
        name: "little box".to_string(),
        size: 1,
        color: "blue".to_string(),
        is_used: false,
        data: 42,
    };

    debug!("little box deref: {:?}", l);
    *l = 420;
    debug!("mut little box deref: {:?}", l);

    log(&MetersSquared(20));
}

fn log(b: &i32) {
    debug!("little box deref is {:?}", b);
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

#[derive(Debug)]
struct LittleBox<T> {
    name: String,
    size: i32,
    color: String,
    is_used: bool,
    data: T,
}

impl<T> Deref for LittleBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for LittleBox<i32> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
