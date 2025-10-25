use std::io;

fn main()
{
    let mut input = String::new();
  
    println!("Enter your age: " );
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:u32 = input.trim().parse().expect("Not a valid number");

    if age >= 40 {
        let incentive = 1_560_000;
        println!("Incentive of employee is N{} per month", incentive);
    }

    else if age >=30 && age < 40 {
        let incentive = 1_480_000;
        println!("Incentive of employee is N{} per month", incentive);
    }

    else if age < 28 && age >=18 {
        let incentive = 1_300_000;
        println!("Incentive of employee is N{} per month", incentive);
    }

    //for inexperienced employees (below 18)
    else if age < 18 {
        let incentive = 100_000;
        println!("Incentive of employee is N{} per month", incentive);
    }

}