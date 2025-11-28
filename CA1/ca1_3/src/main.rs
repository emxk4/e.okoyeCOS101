use std::io;

fn main() {

        println!("----------------------------------------------------");
        println!("|    Code |         Item           |     price(N)  |");
        println!("|    T    |         Tea            |       800     |");
        println!("|    C    |         Coffee         |       1200    |");
        println!("|    S    |         Sandwich       |       2000    |");
        println!("|    J    |         Juice          |       1500    |");

    loop{

        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter the item code (T/C/S/J): ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let item_code = input1.trim().to_uppercase();

        
        let price:f64;

        if item_code == "T" {
            price = 800.0;
        }

        else if item_code == "C" {
            price = 1_200.0;
        }

        else if item_code == "S" {
            price = 2_000.0;
        }

        else if item_code == "J" {
             price = 1_500.0;
        }

        else {
        println!("Invalid item code entered!");
        return;
        }

        let mut total_cost = price * quantity;

        if total_cost >= 5_000.0 {
            let total_cost = total_cost - (total_cost * 0.05);
        }

        println!("The total cost of the item is {}", total_cost);
        
        let mut choice = String::new();
        println!("Would you like to continue?: ");
        let trimmed_choice = input1.trim().to_lowercase();
        if choice == "exit"{
        println!("User quits by inserting {}.", choice );
        break;
        }        
    }
}
