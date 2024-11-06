use std::fs::File;
use std::io::{Result, Write};

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

    for _ in 0..height {
        for x in 0..width {
            let nx = x * max_color / width; // normalized x
            write!(file, "{} {} {}   ", nx, nx, nx)?;
        }
        writeln!(file)?;
    }

    println!("Generated {}x{} PPM image", width, height);

    Ok(())
}
