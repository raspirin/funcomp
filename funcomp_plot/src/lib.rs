use plotters::coord::types::RangedCoordf32;
pub use plotters::prelude::*;

#[macro_export]
macro_rules! draw {
    ($root: expr, $elems: expr) => {{
        let dot = |x: f32, y: f32| {
            EmptyElement::at((x, y)) + Circle::new((0, 0), 1, ShapeStyle::from(&RED).filled())
        };

        for elem in $elems {
            $root.draw(&dot(elem.0 as f32, elem.1 as f32)).unwrap();
        }
    }};
}

pub type Draw<'a> = DrawingArea<BitMapBackend<'a>, Cartesian2d<RangedCoordf32, RangedCoordf32>>;

pub fn get_drawing_area(
    path: &str,
) -> DrawingArea<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>> {
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    root.fill(&RGBColor(250, 250, 250)).unwrap();

    root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..640f32,
        0f32..480f32,
        (20..620, 20..460),
    ))
}
