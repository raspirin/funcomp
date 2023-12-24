use funcomp_plot::{draw, get_drawing_area};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = get_drawing_area("out.png");

    draw!(root, [(0, 0)]);
    root.present()?;
    Ok(())
}
