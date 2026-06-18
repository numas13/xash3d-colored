//! An example program showing how to convert Xash3D color codes to HTML.

use std::fmt::Write;

use xash3d_colored::Colorize;

fn xash_colors() -> String {
    let mut out = String::new();
    let r = "red string".red().once();
    let g = "green string".green();
    write!(&mut out, "{r} and {g}").unwrap();
    out
}

fn html_colors(s: &str) -> String {
    let mut out = String::new();
    for (color, s) in xash3d_colored::str::split(s) {
        if color.is_default() {
            out.push_str(s);
        } else {
            out.push_str("<span style=\"color:");
            out.push_str(color.as_str());
            out.push_str("\">");
            out.push_str(s);
            out.push_str("</span>");
        }
    }
    out
}

fn main() {
    let xash = xash_colors();
    let html = html_colors(&xash);
    println!("Xash: {xash}");
    println!("HTML: {html}");
}
