use rand::{thread_rng, Rng};

/**
 * integration by simulation and finding the area that way
 */
fn monte_carlo_integrate<F>(f: F, a: f64, b: f64, y_min: f64, y_max: f64, num_random: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut rng = thread_rng();
    let total_area = (b - a) * (y_max - y_min); //width*height

    let mut num_under_curve = 0;
    // generate random numbers of a box defined by a->b|a->height
    for _ in 0..num_random {
        let random_x = rng.gen_range(a..b);
        let random_y = rng.gen_range(y_min..y_max);

        // point lines outside the area if y larger than the output y
        let f_y = f(random_x);
        let mut under_curve = random_y <= f_y;
        if random_y < 0.0 {
            under_curve = !under_curve;
        }

        if under_curve {
            num_under_curve += 1;
        }
    }
    let ratio = num_under_curve as f64 / num_random as f64;
    total_area * ratio
}

/**
 * returns an area under the curve from a to b
 *
 * a must be larger than b and define the number of rectangles to approximate with
 *
 * TODO: do trapezoids for faster convergance
 * TODO: parallelize this for large num_rectangles
 * TODO: auto compute the best num_rectangles so still fast and precise
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
    let area2 = monte_carlo_integrate(bell_curve, 0.0, 2.0, -1.0, 1.0, 100_000_0);

    println!("{}", area);
    // \int_{0}^{2} e^{-x^2} dx = 0.8820915739164501
    // supposed to be 0.882081390762, so pretty good!
    println!("{}", area2);
}
