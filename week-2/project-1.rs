fn main() {
    // Project I: Compound Interest Calculation

    // Given values
    let p: f64 = 520_000_000.0; // ₦520,000,000
    let r: f64 = 10.0;                // 10% per annum
    let t: f64 = 5.0;                 // 5 years

    // Formula: A = P * (1 + R/100)^n
    let a = p * (1.0 + (r / 100.0)).powf(t);

    // CI = A - P
    let compound_interest = a - p;

    // Output results
    println!("Project I: Compound Interest Calculation\n");
    println!("Principal (P): ₦{}", p);
    println!("Rate (R): {}%", r);
    println!("Time (n): {} years", t);
    println!("\nAmount (A): ₦{:.2}", a);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}
