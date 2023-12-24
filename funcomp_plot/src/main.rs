use plotters::prelude::*;
use funcomp_plot::{draw, get_drawing_area};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = get_drawing_area("out.png");

    draw!(root, [(1, 1), (2, 2), (3, 300)]);
    root.present()?;
    Ok(())
}
