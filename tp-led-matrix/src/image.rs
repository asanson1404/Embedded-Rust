//! This module allows to manipulate image objects
//! made of pixels

use micromath::F32Ext;
use crate::gamma::gamma_correct;

#[derive(Clone, Copy, Default)]
/// Data structure which represents an individual
/// RGB pixel
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

/// Data strucure which represents a whole 
/// 8x8 image made of pixels
struct Image {

}

impl Color {

    const RED:   Color = Color {r: 255, g: 0, b: 0};
    const GREEN: Color = Color {r: 0, g: 255, b: 0};
    const BLUE:  Color = Color {r: 0, g: 0, b: 255};

    /// Method which applies the gamma_correct correction 
    /// to all components of a color
    pub fn gamma_correct(&self) -> Self {
        Color { r: gamma_correct(self.r), g: gamma_correct(self.g), b: gamma_correct(self.b) }
    }

}