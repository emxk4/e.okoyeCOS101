use std::fs::File;
use std::io::Write;
fn main() {

    let name_of_commisioner = [
        "Aigbogun Alamba Daudu",
        "Murtala Afees Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministry = [
       "Internal Affairs",
       "Justice",
       "Defense",
       "Power & Steel",
       "Petroleum"
    ];  

    let geopolotical_zone = [
        "South West",
        "North West",
        "South South",
        "South West",
        "South East"
    ];

    let mut file = std::fs::File::create("Convicted_minister.txt").expect("create failed");

    writeln!(file,"{:<5} {:<30} {:<20} {:<20}",
        "S/N", "NAME OF COMMISIONER", "MINISTRY", "GEOPOLOTICAL ZONE")
    .expect("Write failed");
   
    for i in 0..name_of_commisioner.len() {
        writeln!(
            file,
            "{:<5} {:<30} {:<20} {:<20}",
            i + 1,
            name_of_commisioner[i],
            ministry[i],
            geopolotical_zone[i]
            );
    }

    println!("File created succesfully")

}