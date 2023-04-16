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
	let area = integrate(bell_curve, a, b, num_iterations);

	println!("{}", area);
	// >> 0.8820915739164501
	// demsmos (below) says 0.882081390762. Pretty darn close.
}
```

In desmos this is what that integration would be evaluated to
<img width="1613" alt="Screenshot 2023-04-15 at 6 50 38 PM" src="https://user-images.githubusercontent.com/65095341/232261859-dc1e5c57-4b48-461f-9a43-49ddbc4c0dc4.png">
