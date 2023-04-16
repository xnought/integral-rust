# Integral Rust

Approximate integration of a function from point a to b for a single variable x.

For example,

```rust
use std::f64::consts::E;

// not normalized because I want to annoy stats people
fn bell_curve(x: f64) -> f64 {
    // e^{-x^2}
    E.powf(-x.powi(2))
}

fn main() {
	// setup for
	// \int_{0}^{2} e^{-x^2} dx
	let a = 0.0;
	let b = 2.0;
	let num_iterations = 100_000;
    let area = integrate(bell_curve, a, b, 100_000);

    println!("{}", area);
    // >> 0.8820915739164501
    // in actuality, it is supposed to be 0.882081390762. Pretty darn good.
}
```
