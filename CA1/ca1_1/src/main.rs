use std::io;

fn main() {

    let mut name = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter the amounts of unit consumed in kwh: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let u:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Customer's name: {}", name);

    println!("--------------------------------------");
    println!("|Units Consumed(kWh)|Rate per unit(N)|");  
    println!("|       0-100       |       20       |");
    println!("|       101-300     |       35       |");
    println!("|    300 and above  |       50       |");
    println!("--------------------------------------");


    if u >= 0.0 && u <= 100.0 {
          let total_bill = u * 20.0;
        println!("The total bill is N{}", total_bill);
    }

    else if u >= 101.0 && u <= 300.0 {
       let total_bill = u * 35.0;
       println!("The total bill is N{}", total_bill);

    }  

    else if u >= 301.0 && u < 500.0 {
        let total_bill = u * 50.0;
        println!("The total bill is N{}", total_bill);
    }  

    else if u > 500.0 {
        let total_bill = (u * 50.0) + 5_000.0;
        println!("The total bill is N{}", total_bill);
    }

}
