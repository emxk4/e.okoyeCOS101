use std::io::Write;

fn main() {
    let breweries = "                     -------------------------------------------------------------------
                     |         Lager         |         Stout        |   Non-Alcoholic   |
                     --------------------------------------------------------------------
                     |       33 Export       |        Legend        |     Maltina       |
                     |      Desperados       |      Turbo King      |   Amstel Malta    |
                     |       Goldberg        |       Williams       |   Malta Gold      |
                     |         Gulder        |                      |    Fayrouz        |
                     |        Heineken       |                      |                   |
                     |          Star         |                      |                   |
                     --------------------------------------------------------------------";
   
    let mut file = std::fs::File::create("Breweries.text").expect("create failed");
    file.write_all("Welcome to Nigerian Plc, the pioneer and larges brewing Company in Nigeria\n"
        .as_bytes()).expect("Write failed");
    file.write_all(breweries.as_bytes()).expect("Write failed");
    println!("Data written to file");

