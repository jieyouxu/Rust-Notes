use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use num::Complex;

use num_cpus;

use image::png::PNGEncoder;
use image::ColorType;

use crossbeam;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    if arguments.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot <FILENAME> <DIMENSIONS> <UPPER_LEFT> <LOWER_RIGHT>"
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.20",
            arguments[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&arguments[2], 'x').expect("failed to parse <DIMENSIONS>");

    println!("[INFO] bounds = {:?}", &bounds);

    let upper_left = parse_complex(&arguments[3]).expect("failed to parse <UPPER_LEFT>");
    let lower_right = parse_complex(&arguments[4]).expect("failed to parse <LOWER_RIGHT>");

    println!("[INFO] upper_left = {:?}", upper_left);
    println!("[INFO] lower_right = {:?}", lower_right);

    let mut pixels_buffer = vec![0; bounds.0 * bounds.1];

    let threads = num_cpus::get();

    println!("[INFO] detected {0} logical cores; spawning {0} threads...", threads);

    let rows_per_stripe = bounds.1 / threads + 1;
    {
        let stripes: Vec<&mut [u8]> = pixels_buffer
            .chunks_mut(rows_per_stripe * bounds.0)
            .collect();
        crossbeam::scope(|spawner| {
            for (i, stripe) in stripes.into_iter().enumerate() {
                let top = rows_per_stripe * i;
                let height = stripe.len() / bounds.0;
                let stripe_bounds = (bounds.0, height);
                let stripe_upper_left = pixel_to_complex(bounds, (0, top), upper_left, lower_right);
                let stripe_lower_right =
                    pixel_to_complex(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(stripe, stripe_bounds, stripe_upper_left, stripe_lower_right);
                });
            }
        })
        .expect("failed to spawn threads");
    }

    write_image(&arguments[1], &pixels_buffer, bounds).expect("failed to write out to PNG file");
}

fn parse_pair<T: FromStr>(s: &str, delimiter: char) -> Option<(T, T)> {
    match s.find(delimiter) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20x", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

// Note: Rust has traits for specifying how types can be converted from
// one to another – this can be an impl instead.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.234,-0.0578"),
        Some(Complex {
            re: 1.234,
            im: -0.0578
        })
    );
    assert_eq!(parse_complex(",-0.94"), None);
}

/// Try to determine whether the complex number `c` is an element of the
/// Mandelbrot set, clamping to at most `limit` iterations to to check.
///
/// If `c` is not an element of the Mandelbrot set, `Some(i)` is returned
/// where `i` is the number of iterations needed to escape the circle
/// (radius 2, centered at origin).
///
/// If `c` is in fact an element of the Mondelbrot set (i.e. we failed to
/// prove that `c` is not an element of the Mandelbrot set within the
/// iteration `limit`), `None` is returned.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

// Note: the function signature is lackluster at best. We can improve it
// by using named structs in favor of anonymous tuples to help readability,
// and `upper_left` and `lower_right` can be grouped together into some
// struct, e.g. `ComplexPlaneConstraint`, as they usually are used together.
fn pixel_to_complex(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    let (col, row) = (pixel.0 as f64, pixel.1 as f64);
    let (img_width, img_height) = (bounds.0 as f64, bounds.1 as f64);

    Complex {
        re: upper_left.re + col * width / img_width,
        // Note that the pixel y coordinate *increases* from top to down, but
        // our view of the complex plane has the y coordinate / imaginary
        // component *decreasing* from top to down.
        im: upper_left.im - row * height / img_height,
    }
}

#[test]
fn test_pixel_to_complex() {
    assert_eq!(
        pixel_to_complex(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    // Precondition: require as many pixels in the buffer as the output
    // image's resolution.
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_complex(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(iterations_count) => 255 - iterations_count as u8,
            }
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let (width, height) = (bounds.0 as u32, bounds.1 as u32);

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, width, height, ColorType::Gray(8))?;

    Ok(())
}
