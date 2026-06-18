use core::fmt;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use crate::Color;

/// A value with a color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Colored<T> {
    pub value: T,
    pub color: Color,
    pub once: bool,
}

impl<T> Colored<T> {
    /// Creates `Colored` with a given value and color.
    pub fn new(value: T, color: Color) -> Self {
        Self {
            value,
            color,
            once: false,
        }
    }

    /// Reset a string color to default after this value.
    pub fn once(mut self) -> Self {
        self.once = true;
        self
    }
}

macro_rules! impl_colored_fmt {
    ($trait:path) => {
        impl<T: $trait> $trait for Colored<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.color.to_code())?;
                self.value.fmt(f)?;
                if self.once {
                    f.write_str(Color::Default.to_code())?;
                }
                Ok(())
            }
        }
    };
}

impl_colored_fmt!(fmt::Binary);
impl_colored_fmt!(fmt::Display);
impl_colored_fmt!(fmt::LowerExp);
impl_colored_fmt!(fmt::LowerHex);
impl_colored_fmt!(fmt::Octal);
impl_colored_fmt!(fmt::Pointer);
impl_colored_fmt!(fmt::UpperExp);
impl_colored_fmt!(fmt::UpperHex);

macro_rules! color_method {
    (fn $meth:ident, $color:expr) => {
        #[inline(always)]
        fn $meth(self) -> Colored<Self::Value> {
            self.color($color)
        }
    };
}

/// A trait for adding colors to values.
pub trait Colorize: Sized {
    type Value: Sized;

    fn color(self, color: Color) -> Colored<Self::Value>;

    color_method!(fn black, Color::Black);
    color_method!(fn red, Color::Red);
    color_method!(fn green, Color::Green);
    color_method!(fn yellow, Color::Yellow);
    color_method!(fn blue, Color::Blue);
    color_method!(fn cyan, Color::Cyan);
    color_method!(fn magenta, Color::Magenta);
    color_method!(fn default, Color::Default);
}

impl<T> Colorize for Colored<T> {
    type Value = T;

    fn color(mut self, color: Color) -> Colored<Self::Value> {
        self.color = color;
        self
    }
}

macro_rules! impl_colorize {
    ($ty:ty) => {
        impl<'a> Colorize for $ty {
            type Value = Self;

            fn color(self, color: Color) -> Colored<Self::Value> {
                Colored::new(self, color)
            }
        }
    };
}

impl_colorize!(&str);

#[cfg(feature = "alloc")]
impl_colorize!(Cow<'_, str>);

impl_colorize!(bool);

impl_colorize!(u8);
impl_colorize!(u16);
impl_colorize!(u32);
impl_colorize!(u64);
impl_colorize!(u128);
impl_colorize!(usize);

impl_colorize!(i8);
impl_colorize!(i16);
impl_colorize!(i32);
impl_colorize!(i64);
impl_colorize!(i128);
impl_colorize!(isize);

impl_colorize!(f32);
impl_colorize!(f64);

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use super::*;

    use alloc::{format, string::String};

    #[test]
    fn colorize_str() {
        assert_eq!(format!("{}", "black".black()), "^0black");
        assert_eq!(format!("{}", "red".red()), "^1red");
        assert_eq!(format!("{}", "green".green()), "^2green");
        assert_eq!(format!("{}", "yellow".yellow()), "^3yellow");
        assert_eq!(format!("{}", "blue".blue()), "^4blue");
        assert_eq!(format!("{}", "cyan".cyan()), "^5cyan");
        assert_eq!(format!("{}", "magenta".magenta()), "^6magenta");
        assert_eq!(format!("{}", "default".default()), "^7default");

        assert_eq!(format!("{}", String::from("abc").black()), "^0abc");

        assert_eq!(format!("{}", "red".red().once()), "^1red^7");
    }

    #[test]
    fn colorize_u32_hex() {
        assert_eq!(format!("{:#8x}", 0xbeef_u32.red()), "^1  0xbeef");
        assert_eq!(format!("{:08x}", 0xdeadbeef_u32.red()), "^1deadbeef");
        assert_eq!(format!("{:#8X}", 0xbeef_u32.red()), "^1  0xBEEF");
        assert_eq!(format!("{:08X}", 0xdeadbeef_u32.red()), "^1DEADBEEF");
    }
}
