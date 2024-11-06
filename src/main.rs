use std::fs::File;
use std::io::{Result, Write};

#[allow(dead_code)]
fn render_mod(x: f32, y: f32) -> (f32, f32, f32) {
    // x and y are normalized [0, 1]
    // here we want value between -1 1
    if x * y >= 0.0 {
        (x, y, 1.0)
    } else {
        (x % y, x % y, x % y)
    }
}

#[allow(dead_code)]
fn render_x(x: f32, y: f32) -> (f32, f32, f32) {
    let _ = y;
    (x, x, x)
}

// http://users.ece.cmu.edu/~adrian/projects/validation/validation.pdf
// https://netpbm.sourceforge.net/doc/ppm.html
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    // We are expecting width and height
    let (width, height) = if args.len() != 3 {
        (300, 300)
    } else {
        let w = args[1].parse::<usize>().unwrap();
        let h = args[2].parse::<usize>().unwrap();
        (w, h)
    };

    let mut file = File::create("output.ppm")?;

    let magic: &str = "P3";
    let max_color: usize = 255;

    // Write the header
    writeln!(file, "{}\n", magic)?;
    writeln!(file, "{} {}\n", width, height)?;
    writeln!(file, "{}\n", max_color)?;

    for y in 0..height {
        let ny: f32 = (y as f32) / (height as f32) * 2.0 - 1.0; // ny -> [-1..1]
        for x in 0..width {
            let nx: f32 = (x as f32) / (width as f32) * 2.0 - 1.0; // nx -> [-1..1]
            let (nr, ng, nb) = render_mod(nx, ny);
            // as nr -> [-1..1]
            //   => r + 1 -> [0..2]
            //   => (r + 1) / 2 -> [0..1]
            //   => 255.0 * (r + 1) -> [0..255]
            let r: usize = ((nr + 1.0) / 2.0 * 255.0) as usize;
            let g: usize = ((ng + 1.0) / 2.0 * 255.0) as usize;
            let b: usize = ((nb + 1.0) / 2.0 * 255.0) as usize;
            write!(file, "{} {} {}   ", r, g, b)?;
        }
        writeln!(file)?;
    }

    println!("Generated {}x{} PPM image", width, height);

    Ok(())
}
