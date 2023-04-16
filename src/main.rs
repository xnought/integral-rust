/**
 * returns an area under the curve from a to b
 *
 * a must be larger than b and define the number of rectangles to approximate with
 *
 * TODO: parallelize this
 * TODO:
 */
fn integrate<F>(f: F, a: f64, b: f64, num_rectangles: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let dx = (b - a) / num_rectangles as f64;
    let mut x = a;
    let mut area_a_to_b = 0.0;

    while x <= b {
        let slice_height = f(x);
        let slice_area = slice_height * dx;

        // accumulate area
        area_a_to_b += slice_area;
        // move along
        x += dx;
    }

    return area_a_to_b;
}

fn main() {
    // e^{-x^2}
    let bell_curve = |x: f64| (-x.powi(2)).exp();
    let area = integrate(bell_curve, 0.0, 2.0, 100_000);

    println!("{}", area);
    // \int_{0}^{2} e^{-x^2} dx = 0.8820915739164501
    // supposed to be 0.882081390762, so pretty good!
}
