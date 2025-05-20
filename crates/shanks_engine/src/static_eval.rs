use shanks_core::board::{Backend, Color};

const MAN_VALUE: f64 = 1.0;
const KING_VALUE: f64 = 3.0;

pub fn static_eval(backend: &Box<dyn Backend>, maximizing_color: Color) -> f64 {
    let score = material_value(backend, maximizing_color);

    score
}

fn material_value(backend: &Box<dyn Backend>, maximizing_color: Color) -> f64 {
    let pvalue = backend.man_count(maximizing_color) as f64 * MAN_VALUE
        + backend.king_count(maximizing_color) as f64 * KING_VALUE;

    let nvalue = backend.man_count(maximizing_color.opposite()) as f64 * MAN_VALUE
        + backend.king_count(maximizing_color.opposite()) as f64 * KING_VALUE;

    pvalue - nvalue
}
