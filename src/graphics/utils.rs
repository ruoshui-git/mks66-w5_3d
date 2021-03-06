use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub(crate) fn create_file(filepath: &str) -> BufWriter<File> {
    let path = Path::new(filepath);
    let display = path.display();
    match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => BufWriter::new(file),
    }
}

pub(crate) fn polar_to_xy(mag: f64, angle_degrees: f64) -> (f64, f64) {
    let (dy, dx) = angle_degrees.to_radians().sin_cos();
    (dx * mag, dy * mag)
}

/// Returns the coefficients of the cubic bezier curve (a, b, c, d)
/// # Arguments
/// `p0`, `p1`, `p2`, `p3` - x or y component of control points
pub(crate) fn compute_bezier3_coef(p0: f64, p1: f64, p2: f64, p3: f64) -> (f64, f64, f64, f64) {
    // (-P0 + 3P1 - 3P2 + P3)t^3 + (3P0 - 6P1 + 3P2)t^2 + (-3P0 + 3P1)t + P0
    // == at^3 + b^2 + ct + d
    (
        -p0 + 3.0 * (p1 - p2) + p3,
        3.0 * p0 - 6.0 * p1 + 3.0 * p2,
        3.0 * (-p0 + p1),
        p0,
    )
}

/// Returns the coefficients of the cubic hermite curve (a, b, c, d)
/// # Arguments
/// `p0`, `p1` - x or y component of endpoints
///
/// `r0`, `r1` - x or y rate of change at each endpoint
pub(crate) fn compute_hermite3_coef(p0: f64, p1: f64, r0: f64, r1: f64) -> (f64, f64, f64, f64) {
    (
        2.0 * (p0 - p1) + r0 + r1,
        3.0 * (-p0 + p1) - 2.0 * r0 - r1,
        r0,
        p0,
    )
}

use crate::graphics::{PPMImg, Matrix};
use std::{fs, process::Command};

pub(crate) fn display_matrix(m: &Matrix, ndc: bool) {
    let mut img = PPMImg::new(500, 500, 225);
    let tmpfile_name = "tmp.ppm";
    if ndc {
        img.render_ndc_edges_n1to1(m);
    }
    else {
        img.render_edge_matrix(m);
    }
    img.write_binary(tmpfile_name)
        .expect("Error writing to file");

    let mut cmd = if cfg!(windows) {
        Command::new("imdisplay")
    } else {
        Command::new("display")
    };
    let mut display = cmd.arg("-flip").arg(tmpfile_name).spawn().unwrap();
    let _result = display.wait().unwrap();
    fs::remove_file(tmpfile_name).expect("Error removing tmp file");
}

/// Returns a mapper function that maps value from one range to another
/// https://stackoverflow.com/a/5732390
pub fn mapper(instart: f64, inend: f64, outstart: f64, outend: f64) -> impl Fn(f64) -> f64 {
    let slope = (outend - outstart) / (inend - instart);
    // move values into closure so they are captured by value, not ref
    move |x| outstart + slope * (x - instart)
}
