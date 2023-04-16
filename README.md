# Integral Rust

Approximate integration of a function from point a to b for a single variable x.

For example,

```rust
// integral of e^{-x^2}
let bell_curve = |x: f64| (-x.powi(2)).exp();

// evaluated from 0 to 2
let from = 0.0;
let to = 2.0;
let num_iterations = 100_000;

let area = integrate(bell_curve, from, to, num_iterations);

println!("{}", area);
// \int_{0}^{2} e^{-x^2} dx = 0.8820915739164501
// supposed to be 0.882081390762, so pretty good!
```

In desmos this is what that integration would be evaluated to
<img width="1613" alt="Screenshot 2023-04-15 at 6 50 38 PM" src="https://user-images.githubusercontent.com/65095341/232261859-dc1e5c57-4b48-461f-9a43-49ddbc4c0dc4.png">
