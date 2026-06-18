//! Utilities for colored string slices.

use core::{iter::FusedIterator, str};

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, string::String};

use crate::{bytes, Color};

/// Returns `true` if a string slice is a [color code](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::is_color_code;
///
/// assert!(is_color_code("^0"));
/// assert!(is_color_code("^3"));
/// assert!(is_color_code("^7"));
///
/// assert!(!is_color_code(""));
/// assert!(!is_color_code("^"));
/// assert!(!is_color_code(" ^1"));
/// assert!(!is_color_code("red"));
/// ```
#[inline(always)]
pub const fn is_color_code(s: &str) -> bool {
    bytes::is_color_code(s.as_bytes())
}

/// Returns `true` if a string slice starts with a [color code](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::starts_with_color;
///
/// assert!(starts_with_color("^0"));
/// assert!(starts_with_color("^1red"));
/// assert!(starts_with_color("^2^3yellow"));
///
/// assert!(!starts_with_color(""));
/// assert!(!starts_with_color("red"));
/// assert!(!starts_with_color("^"));
/// assert!(!starts_with_color(" ^4blue ocean"));
/// ```
#[inline(always)]
pub fn starts_with_color(s: &str) -> bool {
    bytes::starts_with_color(s.as_bytes())
}

/// Returns `true` if a string slice ends with a [color code](crate::Color#color-codes).
///
/// []
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::ends_with_color;
///
/// assert!(ends_with_color("^0"));
/// assert!(ends_with_color("red^1"));
/// assert!(ends_with_color("yellow^3^2"));
///
/// assert!(!ends_with_color(""));
/// assert!(!ends_with_color("red"));
/// assert!(!ends_with_color("^"));
/// assert!(!ends_with_color("blue ocean^4 "));
/// ```
#[inline(always)]
pub fn ends_with_color(s: &str) -> bool {
    bytes::ends_with_color(s.as_bytes())
}

/// Returns a string slice with the color prefix removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::{Color, str::strip_color_prefix};
///
/// assert_eq!(strip_color_prefix(""), None);
/// assert_eq!(strip_color_prefix("^"), None);
/// assert_eq!(strip_color_prefix(" ^0"), None);
/// assert_eq!(strip_color_prefix("^0"), Some((Color::Black, "")));
/// assert_eq!(strip_color_prefix("^0black"), Some((Color::Black, "black")));
/// assert_eq!(strip_color_prefix("^0^1"), Some((Color::Red, "")));
/// assert_eq!(strip_color_prefix("^0^1red"), Some((Color::Red, "red")));
/// assert_eq!(strip_color_prefix("^0^1red^2green"), Some((Color::Red, "red^2green")));
/// ```
#[inline(always)]
pub fn strip_color_prefix(s: &str) -> Option<(Color, &str)> {
    bytes::strip_color_prefix(s.as_bytes()).map(|(color, s)| {
        // SAFETY: `bytes::strip_color_prefix` strips only ASCII characters.
        let s = unsafe { str::from_utf8_unchecked(s) };
        (color, s)
    })
}

/// Returns a string slice with the color suffix removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::{Color, str::strip_color_suffix};
///
/// assert_eq!(strip_color_suffix(""), None);
/// assert_eq!(strip_color_suffix("^"), None);
/// assert_eq!(strip_color_suffix("^0"), Some(("", Color::Black)));
/// assert_eq!(strip_color_suffix("^0^1^2"), Some(("", Color::Green)));
/// assert_eq!(strip_color_suffix("abc^0"), Some(("abc", Color::Black)));
/// assert_eq!(strip_color_suffix("abc^0^1^2"), Some(("abc", Color::Green)));
/// assert_eq!(strip_color_suffix("123^1abc^0"), Some(("123^1abc", Color::Black)));
/// assert_eq!(strip_color_suffix("123^1abc^0^1^2"), Some(("123^1abc", Color::Green)));
/// ```
#[inline(always)]
pub fn strip_color_suffix(s: &str) -> Option<(&str, Color)> {
    bytes::strip_color_suffix(s.as_bytes()).map(|(s, color)| {
        // SAFETY: `bytes::strip_color_suffix` strips only ASCII characters.
        let s = unsafe { str::from_utf8_unchecked(s) };
        (s, color)
    })
}

/// An iterator over colored chunks in a string slice.
pub struct Split<'a> {
    inner: bytes::Split<'a>,
}

impl<'a> Iterator for Split<'a> {
    type Item = (Color, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(color, s)| {
            // SAFETY: `bytes::chunks` splits only ASCII characters.
            let s = unsafe { str::from_utf8_unchecked(s) };
            (color, s)
        })
    }
}

impl<'a> DoubleEndedIterator for Split<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(color, s)| {
            // SAFETY: `bytes::chunks` splits only ASCII characters.
            let s = unsafe { str::from_utf8_unchecked(s) };
            (color, s)
        })
    }
}

impl FusedIterator for Split<'_> {}

/// Creates an iterator over colored chunks in a string slice.
///
/// # Examples
///
/// ```
/// use std::fmt::Write;
/// use xash3d_colored::Color;
///
/// let src = "^1tomato^0 and ^2leaf";
/// let mut buf = String::new();
/// for (color, s) in xash3d_colored::str::split(src) {
///     let color = color.as_str();
///     write!(&mut buf, "<{color}>{s}</{color}>").unwrap();
/// }
/// assert_eq!(buf, "<red>tomato</red><black> and </black><green>leaf</green>");
/// ```
#[inline(always)]
pub const fn split(s: &str) -> Split<'_> {
    Split {
        inner: bytes::split(s.as_bytes()),
    }
}

/// Returns a string slice with leading colors removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::trim_color_start;
///
/// assert_eq!(trim_color_start("foo^2bar"), "foo^2bar");
/// assert_eq!(trim_color_start("^foo^2bar"), "^foo^2bar");
/// assert_eq!(trim_color_start("^1foo^2bar"), "foo^2bar");
/// assert_eq!(trim_color_start("^1^2^3foo^2bar"), "foo^2bar");
/// ```
#[inline]
pub fn trim_color_start(s: &str) -> &str {
    match strip_color_prefix(s) {
        Some((_, s)) => s,
        None => s,
    }
}

/// Returns a string slice with trailing colors removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::trim_color_end;
///
/// assert_eq!(trim_color_end("foo^2bar"), "foo^2bar");
/// assert_eq!(trim_color_end("foo^2bar^"), "foo^2bar^");
/// assert_eq!(trim_color_end("foo^2bar^1"), "foo^2bar");
/// assert_eq!(trim_color_end("foo^2bar^1^2^3"), "foo^2bar");
/// ```
#[inline]
pub fn trim_color_end(s: &str) -> &str {
    match strip_color_suffix(s) {
        Some((s, _)) => s,
        None => s,
    }
}

/// Returns a string slice with leading and trailing colors removed.
#[inline]
pub fn trim_color(s: &str) -> &str {
    trim_color_end(trim_color_start(s))
}

/// Returns `true` if a string slice contains [color codes](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::is_colored;
///
/// assert!(is_colored("^1red"));
/// assert!(is_colored("and ^2green"));
/// assert!(!is_colored("abc"));
/// assert!(!is_colored("123^abc"));
/// ```
#[inline(always)]
pub fn is_colored(s: &str) -> bool {
    bytes::is_colored(s.as_bytes())
}

/// An iterator that returns only text chunks from a string slice.
///
/// See [text].
pub struct Text<'a> {
    inner: Split<'a>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, s)| s)
    }
}

impl DoubleEndedIterator for Text<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(_, s)| s)
    }
}

impl FusedIterator for Text<'_> {}

/// Creates an iterator that returns only text from a string slice.
#[inline(always)]
pub fn text(s: &str) -> Text<'_> {
    Text { inner: split(s) }
}

/// Returns a string without [color codes](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::str::remove_colors;
///
/// assert_eq!(remove_colors("foo^2bar"), "foobar");
/// assert_eq!(remove_colors("^1foo^2bar^3"), "foobar");
/// assert_eq!(remove_colors("^1foo^bar^3"), "foo^bar");
/// assert_eq!(remove_colors("^1foo^2bar^"), "foobar^");
/// assert_eq!(remove_colors("^foo^bar^"), "^foo^bar^");
/// ```
#[cfg(feature = "alloc")]
pub fn remove_colors(s: &str) -> Cow<'_, str> {
    let s = trim_color(s);
    if is_colored(s) {
        let mut out = String::with_capacity(s.len());
        for chunk in text(s) {
            out.push_str(chunk);
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let mut it = split("There is a ^2^1red flower^7 and ^2green grass^0");
        assert_eq!(it.next(), Some((Color::Default, "There is a ")));
        assert_eq!(it.next(), Some((Color::Red, "red flower")));
        assert_eq!(it.next(), Some((Color::Default, " and ")));
        assert_eq!(it.next(), Some((Color::Green, "green grass")));
        assert_eq!(it.next(), Some((Color::Black, "")));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_rev() {
        let mut it = split("There is a ^2^1red flower^7 and ^2green grass^0").rev();
        assert_eq!(it.next(), Some((Color::Black, "")));
        assert_eq!(it.next(), Some((Color::Green, "green grass")));
        assert_eq!(it.next(), Some((Color::Default, " and ")));
        assert_eq!(it.next(), Some((Color::Red, "red flower")));
        assert_eq!(it.next(), Some((Color::Default, "There is a ")));
        assert_eq!(it.next(), None);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn remove_colors_multibyte() {
        assert_eq!(remove_colors("\u{fe0f}^1foo^bar^"), "\u{fe0f}foo^bar^");
        assert_eq!(
            remove_colors("^1^2^3foo\u{fe0f}^2^\u{fe0f}^bar^"),
            "foo\u{fe0f}^\u{fe0f}^bar^"
        );
    }
}
