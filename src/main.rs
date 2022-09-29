use bill::{add, display_menu, get_input, Bill};
use std::{thread, time::Duration};

fn main() {
    loop {
        let mut bills: Vec<Bill> = Vec::new();

        display_menu();

        let input_data = get_input();

        let input_choice = match input_data {
            Ok(result) => result,
            Err(e) => break,
        };

        bills.push(Bill::new("cu".to_string(), 2.0));
        let new_bill = if input_choice == 1 {
            add()
        } else if input_choice == 2 {
            println!("== Bils == ");
            println!("{:?}", bills);
            thread::sleep(Duration::from_millis(4000));
            return ();
        } else {
            return ();
        };

        println!("input_choice >> {:?}", input_choice);

        let bill = Bill::new("s".to_owned(), 23.9);

        println!("{:?}", bill);

        println!("{:?}", input_data);
    }
}
