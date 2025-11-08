fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronical".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 and n3 reference is passed

    // About Electrical/Electronic
    println!(
        "\nThe {} department is informed by the aspiration to
        train electrical/electronic engineering professionals
        in the areas of design, building, and maintenance of
        electrical control systems.",
        n4
    );

    let wk1 = "Computer".to_string();
    let wk2 = "Science".to_string();
    let wk3 = wk1 + &wk2; // wk1 moved, wk2 borrowed

    println!();
    println!(
        "{} is aimed at developing competent, creative,
        innovative, entrepreneurial, and ethically-minded persons,
        capable of creating value in the diverse fields of
        Computer Science.",
        wk3
    );
}
