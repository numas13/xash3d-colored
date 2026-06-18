//! Utilities for colored byte slices.

use core::iter::FusedIterator;

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, vec::Vec};

use crate::Color;

/// Returns `true` if a byte slice is a [color code](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::is_color_code;
///
/// assert!(is_color_code(b"^0"));
/// assert!(is_color_code(b"^3"));
/// assert!(is_color_code(b"^7"));
///
/// assert!(!is_color_code(b""));
/// assert!(!is_color_code(b"^"));
/// assert!(!is_color_code(b" ^1"));
/// assert!(!is_color_code(b"red"));
/// ```
#[inline(always)]
pub const fn is_color_code(s: &[u8]) -> bool {
    Color::from_code(s).is_some()
}

/// Returns `true` if a byte slice starts with a [color code](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::starts_with_color;
///
/// assert!(starts_with_color(b"^0"));
/// assert!(starts_with_color(b"^1red"));
/// assert!(starts_with_color(b"^2^3yellow"));
///
/// assert!(!starts_with_color(b""));
/// assert!(!starts_with_color(b"red"));
/// assert!(!starts_with_color(b"^"));
/// assert!(!starts_with_color(b" ^4blue ocean"));
/// ```
#[inline]
pub fn starts_with_color(s: &[u8]) -> bool {
    s.len() >= 2 && is_color_code(&s[..2])
}

/// Returns `true` if a byte slice ends with a [color code](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::ends_with_color;
///
/// assert!(ends_with_color(b"^0"));
/// assert!(ends_with_color(b"red^1"));
/// assert!(ends_with_color(b"yellow^3^2"));
///
/// assert!(!ends_with_color(b""));
/// assert!(!ends_with_color(b"red"));
/// assert!(!ends_with_color(b"^"));
/// assert!(!ends_with_color(b"blue ocean^4 "));
/// ```
#[inline]
pub fn ends_with_color(s: &[u8]) -> bool {
    s.len() >= 2 && is_color_code(&s[s.len() - 2..])
}

/// Returns a byte slice with the color prefix removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::{Color, bytes::strip_color_prefix};
///
/// assert_eq!(strip_color_prefix(b""), None);
/// assert_eq!(strip_color_prefix(b"^"), None);
/// assert_eq!(strip_color_prefix(b" ^0"), None);
/// assert_eq!(strip_color_prefix(b"^0"), Some((Color::Black, &b""[..])));
/// assert_eq!(strip_color_prefix(b"^0black"), Some((Color::Black, &b"black"[..])));
/// assert_eq!(strip_color_prefix(b"^0^1"), Some((Color::Red, &b""[..])));
/// assert_eq!(strip_color_prefix(b"^0^1red"), Some((Color::Red, &b"red"[..])));
/// assert_eq!(strip_color_prefix(b"^0^1red^2green"), Some((Color::Red, &b"red^2green"[..])));
/// ```
pub fn strip_color_prefix(mut s: &[u8]) -> Option<(Color, &[u8])> {
    let mut color = None;
    while s.len() >= 2 {
        match Color::from_code(&s[..2]) {
            Some(c) => {
                color = Some(c);
                s = &s[2..];
            }
            None => break,
        }
    }
    color.map(|color| (color, s))
}

/// Returns a byte slice with the color suffix removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::{Color, bytes::strip_color_suffix};
///
/// assert_eq!(strip_color_suffix(b""), None);
/// assert_eq!(strip_color_suffix(b"^"), None);
/// assert_eq!(strip_color_suffix(b"^0"), Some((&b""[..], Color::Black)));
/// assert_eq!(strip_color_suffix(b"^0^1^2"), Some((&b""[..], Color::Green)));
/// assert_eq!(strip_color_suffix(b"abc^0"), Some((&b"abc"[..], Color::Black)));
/// assert_eq!(strip_color_suffix(b"abc^0^1^2"), Some((&b"abc"[..], Color::Green)));
/// assert_eq!(strip_color_suffix(b"123^1abc^0"), Some((&b"123^1abc"[..], Color::Black)));
/// assert_eq!(strip_color_suffix(b"123^1abc^0^1^2"), Some((&b"123^1abc"[..], Color::Green)));
/// ```
pub fn strip_color_suffix(mut s: &[u8]) -> Option<(&[u8], Color)> {
    let mut color = None;
    while s.len() >= 2 {
        match Color::from_code(&s[s.len() - 2..]) {
            Some(c) => {
                if color.is_none() {
                    color = Some(c);
                }
                s = &s[..s.len() - 2];
            }
            None => break,
        }
    }
    color.map(|color| (s, color))
}

/// An iterator over colored chunks in a byte slice.
pub struct Split<'a> {
    inner: &'a [u8],
}

impl<'a> Iterator for Split<'a> {
    type Item = (Color, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let (color, text) = strip_color_prefix(self.inner).unwrap_or((Color::Default, self.inner));
        let mut offset = 0;
        while offset < text.len() {
            match text[offset..].iter().position(|i| *i == b'^') {
                Some(p) => {
                    offset += p;
                    if starts_with_color(&text[offset..]) {
                        break;
                    }
                    offset += 1;
                }
                None => offset = text.len(),
            }
        }

        let (head, tail) = text.split_at(offset);
        self.inner = tail;
        Some((color, head))
    }
}

impl DoubleEndedIterator for Split<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        for i in (0..self.inner.len() - 1).rev() {
            let (head, tail) = self.inner.split_at(i);
            if let Some((color, tail)) = strip_color_prefix(tail) {
                self.inner = trim_color_end(head);
                return Some((color, tail));
            }
        }

        let text = self.inner;
        self.inner = b"";
        Some((Color::Default, text))
    }
}

impl FusedIterator for Split<'_> {}

/// Creates an iterator over colored chunks in a byte slice.
///
/// # Examples
///
/// ```
/// use std::{io::Write, str};
/// use xash3d_colored::Color;
///
/// let src = b"^1flower^0 and ^2grass";
/// let mut buf = Vec::new();
/// for (color, s) in xash3d_colored::bytes::split(src) {
///     let color = color.as_str();
///     write!(&mut buf, "<{color}>").unwrap();
///     buf.extend_from_slice(s);
///     write!(&mut buf, "</{color}>").unwrap();
/// }
/// assert_eq!(buf, &b"<red>flower</red><black> and </black><green>grass</green>"[..]);
/// ```
#[inline(always)]
pub const fn split(s: &[u8]) -> Split<'_> {
    Split { inner: s }
}

/// Returns a byte slice with leading colors removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::trim_color_start;
///
/// assert_eq!(trim_color_start(b"foo^2bar"), b"foo^2bar");
/// assert_eq!(trim_color_start(b"^foo^2bar"), b"^foo^2bar");
/// assert_eq!(trim_color_start(b"^1foo^2bar"), b"foo^2bar");
/// assert_eq!(trim_color_start(b"^1^2^3foo^2bar"), b"foo^2bar");
/// ```
#[inline]
pub fn trim_color_start(s: &[u8]) -> &[u8] {
    match strip_color_prefix(s) {
        Some((_, s)) => s,
        None => s,
    }
}

/// Returns a byte slice with trailing colors removed.
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::trim_color_end;
///
/// assert_eq!(trim_color_end(b"foo^2bar"), &b"foo^2bar"[..]);
/// assert_eq!(trim_color_end(b"foo^2bar^"), &b"foo^2bar^"[..]);
/// assert_eq!(trim_color_end(b"foo^2bar^1"), &b"foo^2bar"[..]);
/// assert_eq!(trim_color_end(b"foo^2bar^1^2^3"), &b"foo^2bar"[..]);
/// ```
#[inline]
pub fn trim_color_end(s: &[u8]) -> &[u8] {
    match strip_color_suffix(s) {
        Some((s, _)) => s,
        None => s,
    }
}

/// Returns a byte slice with leading and trailing colors removed.
#[inline]
pub fn trim_color(s: &[u8]) -> &[u8] {
    trim_color_end(trim_color_start(s))
}

/// Returns `true` if a byte slice contains [color codes](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::is_colored;
///
/// assert!(is_colored(b"^1red"));
/// assert!(is_colored(b"and ^2green"));
/// assert!(!is_colored(b"abc"));
/// assert!(!is_colored(b"123^abc"));
/// ```
pub fn is_colored(s: &[u8]) -> bool {
    for i in 0..s.len() {
        if starts_with_color(&s[i..]) {
            return true;
        }
    }
    false
}

/// An iterator that returns only text chunks from a byte slice.
///
/// See [text].
pub struct Text<'a> {
    inner: Split<'a>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, s)| s)
    }
}

impl<'a> DoubleEndedIterator for Text<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(_, s)| s)
    }
}

impl FusedIterator for Text<'_> {}

/// Creates an iterator that returns only text from a byte slice.
#[inline(always)]
pub fn text(s: &[u8]) -> Text<'_> {
    Text { inner: split(s) }
}

/// Returns a byte slice without [color codes](crate::Color#color-codes).
///
/// # Examples
///
/// ```
/// use xash3d_colored::bytes::remove_colors;
///
/// assert_eq!(remove_colors(b"foo^2bar"), &b"foobar"[..]);
/// assert_eq!(remove_colors(b"^1foo^2bar^3"), &b"foobar"[..]);
/// assert_eq!(remove_colors(b"^1foo^bar^3"), &b"foo^bar"[..]);
/// assert_eq!(remove_colors(b"^1foo^2bar^"), &b"foobar^"[..]);
/// assert_eq!(remove_colors(b"^foo^bar^"), &b"^foo^bar^"[..]);
/// ```
#[cfg(feature = "alloc")]
pub fn remove_colors(s: &[u8]) -> Cow<'_, [u8]> {
    let s = trim_color(s);
    if is_colored(s) {
        let mut out = Vec::with_capacity(s.len());
        for chunk in text(s) {
            out.extend_from_slice(chunk);
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
        let mut it = split(b"There is a ^2^1red flower^7 and ^2green grass^0");
        assert_eq!(it.next(), Some((Color::Default, &b"There is a "[..])));
        assert_eq!(it.next(), Some((Color::Red, &b"red flower"[..])));
        assert_eq!(it.next(), Some((Color::Default, &b" and "[..])));
        assert_eq!(it.next(), Some((Color::Green, &b"green grass"[..])));
        assert_eq!(it.next(), Some((Color::Black, &b""[..])));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_rev() {
        let mut it = split(b"There is a ^2^1red flower^7 and ^2green grass^0").rev();
        assert_eq!(it.next(), Some((Color::Black, &b""[..])));
        assert_eq!(it.next(), Some((Color::Green, &b"green grass"[..])));
        assert_eq!(it.next(), Some((Color::Default, &b" and "[..])));
        assert_eq!(it.next(), Some((Color::Red, &b"red flower"[..])));
        assert_eq!(it.next(), Some((Color::Default, &b"There is a "[..])));
        assert_eq!(it.next(), None);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn remove_colors_multibyte() {
        assert_eq!(
            remove_colors(b"\xfe\x0f^1foo^bar^"),
            &b"\xfe\x0ffoo^bar^"[..]
        );
        assert_eq!(
            remove_colors(b"^1^2^3foo\xfe\x0f^2^\xfe\x0f^bar^"),
            &b"foo\xfe\x0f^\xfe\x0f^bar^"[..]
        );
    }
}
