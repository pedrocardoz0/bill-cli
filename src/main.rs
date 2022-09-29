use bill::{display_menu, get_input, Bill};
fn main() {
    loop {
        display_menu();

        let input_data = get_input();

        let input_choice = match input_data {
            Ok(result) => result,
            Err(e) => break,
        };

        println!("input_choice >> {:?}", input_choice);

        let bill = Bill::new("s".to_owned(), 23.9);

        println!("{:?}", bill);

        println!("{:?}", input_data);
    }
}
