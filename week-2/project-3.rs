fn main() {
    // Project IV: Depreciation Calculation
    // Ms. Anjola Olowokere's TV depreciation over 3 years

    // Given values
    let p: f64 = 510_000.0; // initial value (₦)
    let r: f64 = 5.0;       // depreciation rate per annum (%)
    let n: f64 = 3.0;       // time in years

    // Formula: A = P * [1 - (R/100)]^n
    let a = p * (1.0 - (r / 100.0)).powf(n);

    // Display results
    println!("Ms. Anjola Olowokere - TV Depreciation Project\n");
    println!("Initial Value (P): ₦{:.2}", p);
    println!("Rate of Depreciation (R): {:.2}%", r);
    println!("Time (n): {:.0} years", n);
    println!("\nValue of TV after {:.0} years: ₦{:.2}", n, a);
}
