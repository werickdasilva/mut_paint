use std::f64;
use crate::{
    core::{canvas::Canvas, event::AppEvents},
};

pub struct RotateTool {
    step: f64,
}

impl RotateTool {
    pub fn new() -> Self {
        Self {
            step: f64::consts::PI / 36.0, //5 degrees
        }
    }

    pub fn rotate_left(&self, canvas: &mut Canvas) {
        canvas.rotation -= self.step
    }
    pub fn rotate_right(&self, canvas: &mut Canvas) {
           canvas.rotation += self.step
    }
}
