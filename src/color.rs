use core::{fmt, str::FromStr};

const COLOR_NAME_MIN: usize = 3;
const COLOR_NAME_MAX: usize = 7;

/// Xash3D string colors.
///
/// # Color codes
///
/// | Code | Color   |
/// | ---- | ------- |
/// | `^0` | black   |
/// | `^1` | red     |
/// | `^2` | green   |
/// | `^3` | yellow  |
/// | `^4` | blue    |
/// | `^5` | cyan    |
/// | `^6` | magenta |
/// | `^7` | default |
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Magenta,
    Default,
    // Do not forget to add new variants to enumerate function.
}

impl Color {
    /// Returns a slice with all colors.
    pub const fn enumerate() -> &'static [Self] {
        &[
            Self::Black,
            Self::Red,
            Self::Green,
            Self::Yellow,
            Self::Blue,
            Self::Cyan,
            Self::Magenta,
            Self::Default,
        ]
    }

    /// Creates `Color` from a color code in a given byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use xash3d_colored::Color;
    ///
    /// assert_eq!(Color::from_code(b"^0"), Some(Color::Black));
    /// assert_eq!(Color::from_code(b"^1"), Some(Color::Red));
    /// assert_eq!(Color::from_code(b"^2"), Some(Color::Green));
    /// assert_eq!(Color::from_code(b"^3"), Some(Color::Yellow));
    /// assert_eq!(Color::from_code(b"^4"), Some(Color::Blue));
    /// assert_eq!(Color::from_code(b"^5"), Some(Color::Cyan));
    /// assert_eq!(Color::from_code(b"^6"), Some(Color::Magenta));
    /// assert_eq!(Color::from_code(b"^7"), Some(Color::Default));
    ///
    /// assert_eq!(Color::from_code(b"^0 "), None);
    /// assert_eq!(Color::from_code(b"^4a"), None);
    /// assert_eq!(Color::from_code(b"^green"), None);
    /// assert_eq!(Color::from_code(b"cyan"), None);
    /// ```
    pub const fn from_code(s: &[u8]) -> Option<Self> {
        match s {
            b"^0" => Some(Self::Black),
            b"^1" => Some(Self::Red),
            b"^2" => Some(Self::Green),
            b"^3" => Some(Self::Yellow),
            b"^4" => Some(Self::Blue),
            b"^5" => Some(Self::Cyan),
            b"^6" => Some(Self::Magenta),
            b"^7" => Some(Self::Default),
            // Xash3D engine has this logic.
            b"^8" => Some(Self::Black),
            b"^9" => Some(Self::Red),
            _ => None,
        }
    }

    /// Returns a color code.
    ///
    /// # Examples
    ///
    /// ```
    /// use xash3d_colored::Color;
    ///
    /// assert_eq!("^0", Color::Black.to_code());
    /// assert_eq!("^1", Color::Red.to_code());
    /// assert_eq!("^2", Color::Green.to_code());
    /// assert_eq!("^3", Color::Yellow.to_code());
    /// assert_eq!("^4", Color::Blue.to_code());
    /// assert_eq!("^5", Color::Cyan.to_code());
    /// assert_eq!("^6", Color::Magenta.to_code());
    /// assert_eq!("^7", Color::Default.to_code());
    /// ```
    pub const fn to_code(&self) -> &'static str {
        match self {
            Self::Black => "^0",
            Self::Red => "^1",
            Self::Green => "^2",
            Self::Yellow => "^3",
            Self::Blue => "^4",
            Self::Cyan => "^5",
            Self::Magenta => "^6",
            Self::Default => "^7",
        }
    }

    /// Creates `Color` from a color name.
    ///
    /// # Examples
    ///
    /// ```
    /// use xash3d_colored::Color;
    ///
    /// assert_eq!(Color::from_name(b"black"), Some(Color::Black));
    /// assert_eq!(Color::from_name(b"red"), Some(Color::Red));
    /// assert_eq!(Color::from_name(b"green"), Some(Color::Green));
    /// assert_eq!(Color::from_name(b"yellow"), Some(Color::Yellow));
    /// assert_eq!(Color::from_name(b"blue"), Some(Color::Blue));
    /// assert_eq!(Color::from_name(b"cyan"), Some(Color::Cyan));
    /// assert_eq!(Color::from_name(b"magenta"), Some(Color::Magenta));
    /// assert_eq!(Color::from_name(b"white"), Some(Color::Default));
    /// assert_eq!(Color::from_name(b"default"), Some(Color::Default));
    ///
    /// assert_eq!(Color::from_name(b"Black"), None);
    /// assert_eq!(Color::from_name(b"WHITE"), None);
    /// ```
    pub const fn from_name(s: &[u8]) -> Option<Self> {
        match s {
            b"black" => Some(Self::Black),
            b"red" => Some(Self::Red),
            b"green" => Some(Self::Green),
            b"yellow" => Some(Self::Yellow),
            b"blue" => Some(Self::Blue),
            b"cyan" => Some(Self::Cyan),
            b"magenta" => Some(Self::Magenta),
            b"white" => Some(Self::Default),
            b"default" => Some(Self::Default),
            _ => None,
        }
    }

    /// Creates `Color` from a color name ignoring case.
    ///
    /// # Examples
    ///
    /// ```
    /// use xash3d_colored::Color;
    ///
    /// assert_eq!(Color::from_name_ignore_case(b"Black"), Some(Color::Black));
    /// assert_eq!(Color::from_name_ignore_case(b"Red"), Some(Color::Red));
    /// assert_eq!(Color::from_name_ignore_case(b"Green"), Some(Color::Green));
    /// assert_eq!(Color::from_name_ignore_case(b"Yellow"), Some(Color::Yellow));
    /// assert_eq!(Color::from_name_ignore_case(b"Blue"), Some(Color::Blue));
    /// assert_eq!(Color::from_name_ignore_case(b"Cyan"), Some(Color::Cyan));
    /// assert_eq!(Color::from_name_ignore_case(b"Magenta"), Some(Color::Magenta));
    /// assert_eq!(Color::from_name_ignore_case(b"White"), Some(Color::Default));
    /// assert_eq!(Color::from_name_ignore_case(b"Default"), Some(Color::Default));
    ///
    /// assert_eq!(Color::from_name_ignore_case(b"tree"), None);
    /// assert_eq!(Color::from_name_ignore_case(b" green"), None);
    /// assert_eq!(Color::from_name_ignore_case(b"Yellow "), None);
    /// ```
    pub fn from_name_ignore_case(s: &[u8]) -> Option<Self> {
        if s.len() < COLOR_NAME_MIN || s.len() > COLOR_NAME_MAX {
            return None;
        }
        let buf = &mut [0; COLOR_NAME_MAX][..s.len()];
        buf.copy_from_slice(s);
        buf.make_ascii_lowercase();
        Self::from_name(buf)
    }

    /// Returns a color name.
    ///
    /// # Examples
    ///
    /// ```
    /// use xash3d_colored::Color;
    ///
    /// assert_eq!(Color::Green.as_str(), "green");
    /// assert_eq!(Color::Cyan.as_str(), "cyan");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::Red => "red",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Cyan => "cyan",
            Self::Magenta => "magenta",
            Self::Default => "white",
        }
    }

    /// Returns `true` if it is a default color.
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s.as_bytes()).ok_or(())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.to_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_names_len() {
        for color in Color::enumerate() {
            let name = color.as_str();
            assert!(name.len() >= COLOR_NAME_MIN, "{:?} unexpected len", name);
            assert!(name.len() <= COLOR_NAME_MAX, "{:?} unexpected len", name);
        }
    }

    #[test]
    fn color_names() {
        for color in Color::enumerate() {
            let name = color.as_str();
            Color::from_name(name.as_bytes()).unwrap();
        }
    }

    #[test]
    fn wrap_color_codes() {
        assert_eq!(Color::from_code(b"^8"), Color::from_code(b"^0"));
        assert_eq!(Color::from_code(b"^9"), Color::from_code(b"^1"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn color_fmt() {
        use alloc::format;

        assert_eq!(format!("{}cyan", Color::Cyan), "^5cyan");
    }
}
