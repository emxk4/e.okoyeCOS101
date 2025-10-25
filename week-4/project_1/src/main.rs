use std::io;

fn main()
{
    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut inputa).expect("Not a valid string");
    let a:f64 = inputa.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut inputb).expect("Not a valid string");
    let b:f64 = inputb.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut inputc).expect("Not a valid string");
    let c:f64 = inputc.trim().parse().expect("Not a valid number");

    //discrimant; d = b ^ 2 - 4 * a * c
    let d = b * b - 4.0 * a * c;

    if d > 0.0
    {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b +d.sqrt())  / (2.0 * a);
        println!("2 distinct roots: {:.3} and {:.3}",root1, root2);
    }
     
    else if d == 0.0
    {
        let root = -b / (2.0 * a);
        println!("1 distinct root: {:.3}",root);
    }
    
    else if d < 0.0
    {
        println!("No distinct roots. the discriminant is -ve");
     }


}