use std::io;

#[derive(Debug)]
pub struct Bill {
    name: String,
    amount: f64,
}

impl Bill {
    pub fn new(name: String, amount: f64) -> Self {
        Self { name, amount }
    }
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn add() -> Bill {
    clear();
    println!("= Adding bill =");

    let mut buffer_name = String::new();
    let mut buffer_amout = String::new();

    println!("Name:");
    io::stdin().read_line(&mut buffer_name).unwrap();

    println!("Amount:");
    io::stdin().read_line(&mut buffer_amout).unwrap();

    let amount: f64 = buffer_amout.trim().parse::<f64>().unwrap();
    dbg!(&amount);
    Bill::new(buffer_name, amount)
}

pub fn remove() {
    clear();
    println!("Removing bill");
}

pub fn edit() {
    clear();
    println!("Editing bill");
}

pub fn display_menu() {
    clear();
    println!("== Manage Bills ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("5. Bill total");
}

pub fn get_input() -> Result<u32, String> {
    let mut buffer = String::new();

    println!("\nEnter selection:");
    io::stdin().read_line(&mut buffer).unwrap();

    let buffer_string = buffer.as_str().trim();

    match buffer_string {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        "4" => Ok(4),
        "5" => Ok(5),
        _ => Err("Faild".to_string()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
