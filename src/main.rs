use bill::{add, display_menu, get_input, Bill};
use std::{thread, time::Duration};

fn main() {
    let mut bills: Vec<Bill> = Vec::new();

    loop {
        display_menu();

        let input_data = get_input();

        let input_choice = match input_data {
            Ok(result) => result,
            Err(e) => break,
        };

        if input_choice == 1 {
            let new_bill = add();
            dbg!(&new_bill);
            bills.push(new_bill);
        }

        if input_choice == 2 {
            println!("== Bils == ");
            println!("{:?}", bills);
            thread::sleep(Duration::from_millis(4000));
        }

        println!("input_choice >> {:?}", input_choice);

        let bill = Bill::new("s".to_owned(), 23.9);

        println!("{:?}", bill);

        println!("{:?}", input_data);
    }
}
