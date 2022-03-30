#![no_std]

pub mod image;
pub mod gamma;

/// Reexporting types
pub use image::{Color, Image};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
