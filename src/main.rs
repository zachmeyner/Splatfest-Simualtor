use std::io;

fn main() {
    println!("Select simulation type\n1) Basic (in implementation)\n2) Basic+ (not implemented)\n3) Tricolor (not implemented)");

    loop {
        let mut select = String::new();
        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        let select = select.trim();

        println!("{:?}", select);
        break match select {
            "1" => basic_sim(),
            "2" => {
                println!("Not yet implemented");
                continue;
            }
            "3" => {
                println!("Not yet implemented");
                continue;
            }
            _ => {
                println!("Please select simulation type");
                continue;
            }
        };
    }
}

fn basic_sim() {
    println!("Enter population of each team (% to second decimal)");
}
