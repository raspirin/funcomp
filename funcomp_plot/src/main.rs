use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("out.png", (640, 480)).into_drawing_area();

    root.fill(&RGBColor(250, 250, 250))?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    ));

    let dot_and_label = |x: f32, y: f32| {
        EmptyElement::at((x, y)) + Circle::new((0, 0), 2, ShapeStyle::from(&BLACK).filled())
    };

    root.draw(&dot_and_label(0.5, 0.6))?;
    root.draw(&dot_and_label(0.25, 0.33))?;
    root.draw(&dot_and_label(0.8, 0.8))?;
    root.present()?;
    Ok(())
}
