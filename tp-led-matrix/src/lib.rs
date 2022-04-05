#![no_std]

pub mod image;
pub mod gamma;
pub mod matrix;

/// Reexporting types
pub use image::{Color, Image};
pub use matrix::Matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
