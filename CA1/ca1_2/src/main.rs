use std::io;

fn main() {

    loop{

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value for P in N: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value for R in %: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let r:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value for T in years: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let t:f64 = input3.trim().parse().expect("Not a valid number");

    //Calculate the amount

    let a = p * (1.0 + (r /100.0) );
    println!("The amount is {}", a);

    println!("Would you like to continue(y/n)? : ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid string");

    if choice == "n" {
         println!("Program ended.");
         break;
        }
   
   
    }
}