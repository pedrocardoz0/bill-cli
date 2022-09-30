use std::io;

#[derive(Debug)]
pub struct Bill {
    pub id: usize,
    pub name: String,
    pub amount: f64,
}

impl Bill {
    pub fn new(id: usize, name: String, amount: f64) -> Self {
        Self { id, name, amount }
    }
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn add(len: usize) -> Bill {
    clear();
    println!("= Adding bill =");

    let mut buffer_name = String::new();
    let mut buffer_amout = String::new();

    println!("Name:");
    io::stdin().read_line(&mut buffer_name).unwrap();

    println!("Amount:");
    io::stdin().read_line(&mut buffer_amout).unwrap();

    let amount: f64 = buffer_amout.trim().parse::<f64>().unwrap();

    let id = len + 1;

    Bill::new(
        id.try_into().unwrap(),
        buffer_name.trim().to_string(),
        amount,
    )
}

pub fn remove(bills: &mut Vec<Bill>) -> Result<bool, String> {
    clear();
    println!("Removing bill");

    let mut buffer_id = String::new();

    println!("ID:");
    io::stdin().read_line(&mut buffer_id).unwrap();

    let id = match buffer_id.trim().parse::<usize>() {
        Ok(result) => Some(result),
        Err(_) => None,
    };

    let id = match id {
        Some(_id) => _id,
        None => 0,
    };

    let index: isize = match bills.iter().position(|item| item.id == id) {
        Some(_index) => _index.try_into().unwrap(),
        None => -1,
    };

    if index > -1 {
        let _ = &bills.remove(index.try_into().unwrap());
        Ok(true)
    } else {
        Err("faild".to_string())
    }
}

pub fn edit(bills: &mut Vec<Bill>) -> Result<bool, String> {
    clear();
    println!("Editing bill");

    let mut buffer_id = String::new();

    println!("ID:");
    io::stdin().read_line(&mut buffer_id).unwrap();

    let id = match buffer_id.trim().parse::<isize>() {
        Ok(id) => id,
        Err(_) => -1,
    };

    if id > -1 {
        let index: isize = match bills
            .iter()
            .position(|item| item.id == id.try_into().unwrap())
        {
            Some(_index) => _index.try_into().unwrap(),
            None => -1,
        };

        if index > -1 {
            let _ = &bills.remove(index.try_into().unwrap());
            Ok(true)
        } else {
            Err("No index".to_string())
        }
    } else {
        Err("Invalid input".to_string())
    }
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
