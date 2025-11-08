use std::io;

fn main() {

    println!("----------------------------------------------------------------------");
    println!("|Menu Code|                 MENU                  |     Price         ");
    println!("----------------------------------------------------------------------");   
    println!("|   P     |       Pounded Yam/Edinkaiko Soup      |     3,200         ");
    println!("|   F     |        Fried Rice and Chicken         |     3,000         ");
    println!("|   A     |        Amala and Ewedu Soup           |     2,500         ");
    println!("|   E     |         Eba and Egusi Soup            |     2,000         ");
    println!("|   W     |         White rice and Stew           |     2,500         ");
    println!("----------------------------------------------------------------------");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the Menu Code (P/F/A/E/W): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let menu_code = input1.trim().to_uppercase();

    println!("Enter the quantity you desire: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f64 = input2.trim().parse().expect("Not a valid number");

    let price:f64;

    if menu_code == "P"{
        price = 3_200.0;
    }

    else if menu_code == "F"{
        price = 3_000.0;
    }

    else if menu_code == "A"{
        price = 2_500.0;
    }

    else if menu_code == "E"{
        price = 2_000.0;
    }

    else if menu_code == "W"{
        price = 2_500.0;
    }

    else{
    println!("Invalid item code entered");
    return;
    }

    let mut total_order = price * quantity;

    if total_order > 10_000.0{
        let total_order = total_order - (total_order * 0.05);
    }

    println!("The total order is summed up to {}", total_order);

}