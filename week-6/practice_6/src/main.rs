fn main() {
   
   let n1 = "Electrical".to_string();
   let n2 = "Electronical".to_string();
   let n3 = n1 + &n2 + &n3; // n2 & n3 reference is passed

   //About Electrical/Electronic
   println!("\nThe {} is informed by the aspiration to
    train electrical/electronic engineering proffesionals
    in the areas of design, building and maintenance of
    electrical control systems,", n4);

   let wk1 = "Computer".to_string();
   let wk2 = "Science".to_string();
   let wk3 = wk1 + &wk2;    //wk2 reference is passed
   println!();
   println!("{} is aimed at developing competent, creative,
    innovative, entrepreneurial and ethically-minded persons,
    capable of creating value in the diverse fields of 
    Computer Science. ",wk3);
}
