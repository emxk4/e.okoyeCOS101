use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {

    let students = [
        ["\nOluchi Mordi", "ACC10211111", "Accounting", "300"],
        ["Adams Aliyu", "ECO10110101", "Economics", "100"],
        ["Shania Bolade", "CSC10328828", "Computer", "200"],
        ["Adekunle Gold", "EEE11020202", "Electrical", "200"],
        ["Blanca Edemoh", "MEE1020202001", "Mechanicl", "100"],
    ];

    let mut file = File::create("pau_isms.txt").expect("create failed");
    file.write_all("PAU SMIS\n".as_bytes()).expect("write failed");
    file.write_all("Student Name\tMatric Number\tDepartment\tLevel".as_bytes()).expect("write failed");

    for  stu in students {
        let arrays = format!("{}\t{}\t{}\t{}\n", stu[0], stu[1], stu[2], stu[3]);
        file.write_all(arrays.as_bytes()).expect("write failed");
    }

    let mut file = std::fs::File::open("pau_isms.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}