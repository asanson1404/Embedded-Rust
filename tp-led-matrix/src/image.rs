//! This module allows to manipulate image objects
//! made of pixels

use micromath::F32Ext;
use core::ops::Mul;
use core::ops::Div;
use crate::gamma::gamma_correct;

#[derive(Clone, Copy, Default)]
/// Data structure which represents an individual
/// RGB pixel
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

/// Data strucure which represents a whole 
/// 8x8 image made of pixels
pub struct Image ([Color; 64]);

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

impl Mul<f32> for Color {

    type Output = Self;

    /// Function which returns another Color whose individual components are multiplied
    /// by the given floating point value (rhs). Each component will stay in range of an u8.
    fn mul(self, rhs: f32) -> Self {
        Color { r: u8_range((self.r as f32 * rhs).round()),
                g: u8_range((self.g as f32 * rhs).round()),
                b: u8_range((self.b as f32 * rhs).round()) }
    }

}

impl Div<f32> for Color {

    type Output = Self;

    /// Function which returns another Color whose individual components are divided
    /// by the given floating point value (rhs). Each component will stay in range of an u8
    fn div(self, rhs: f32) -> Self {
        Color::mul(self, 1.0/rhs)
    }
}

impl Image {

    /// Function which returns an image filled with 
    /// the color given as an argument
    pub fn new_solid(color: Color) -> Self {
        Image([color; 64])
    }

}

pub trait Default {
    fn default() -> Self;
}

/// This trait will have a default function to return an image
/// filled with the default color (BLACK)
impl Default for Image {

    fn default() -> Self {
        Image::new_solid(Color{r:0, g:0, b:0})
    }

}

/// Fonction to be sure a Color component will stay within
/// the range of an u8 after a multiplication or a division
fn u8_range (f: f32) -> u8 {
    if f > 255.0 {u8::MAX}
    else if f < 0.0 {u8::MIN}
    else {f as u8}
}