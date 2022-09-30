use bill::{add, display_menu, get_input, remove, Bill};
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
            let lenght = bills.len();
            let new_bill = add(lenght);
            bills.push(new_bill);
        }

        if input_choice == 2 {
            println!("== Bils == ");
            println!("{:?}", bills);
            thread::sleep(Duration::from_millis(4000));
        }

        if input_choice == 3 {
            println!("== Removing ==");
            let id = remove();

            let index = bills
                .iter()
                .position(|item| item.id == id)
                .expect("No item at this ID");

            println!("Removing...");
            thread::sleep(Duration::from_millis(4000));
            bills.remove(index);
        }

        println!("input_choice >> {:?}", input_choice);

        let bill = Bill::new(1, "s".to_owned(), 23.9);

        println!("{:?}", bill);

        println!("{:?}", input_data);
    }
}
