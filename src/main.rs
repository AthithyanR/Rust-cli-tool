use std::collections::HashMap;

enum MainMenu {
    AddEdit,
    View,
    Remove,
    Undo
}

impl MainMenu {
    fn from_str(str: &str) -> Option<MainMenu> {
        match str {
            "1" => Some(MainMenu::AddEdit),
            "2" => Some(MainMenu::View),
            "3" => Some(MainMenu::Remove),
            "4" => Some(MainMenu::Undo),
            _ => None
        }
    }
    fn show() {
        println!("== Manage bills ==");
        println!("1. Add/Edit bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Undo bill change");
        println!("Enter selection: ");
    }
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f32,
}

pub struct Bills {
    inner: HashMap<String, Vec<f32>>
}

impl Bills {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }
    fn add(&mut self, bill: Bill) {
        let amounts = self.inner.entry(bill.name).or_insert_with(|| vec![]);
        amounts.push(bill.amount);
    }
}

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while std::io::stdin().read_line(&mut buffer).is_err() {
        println!("Enter valid input");
    };
    buffer = buffer.trim().to_owned();
    if buffer == "" {
        return None;
    }
    Some(buffer)
}

pub fn get_input_amount() -> Option<f32> {
    loop {
        let mut buffer = String::new();
        while std::io::stdin().read_line(&mut buffer).is_err() {
            println!("Enter valid input");
        };
        buffer = buffer.trim().to_owned();
        if buffer == "" {
            return None;
        }
        let parsed_buffer: Result<f32, _> = buffer.parse();
        match parsed_buffer {
            Ok(v) => return Some(v),
            Err(_) => println!("Please enter a number")
        }
    }
}

mod menu {
    use crate::{Bills, Bill, get_input, get_input_amount};

    pub fn add_or_edit_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(v) => v,
            None => return,
        };
        println!("Amount: ");
        let amount = match get_input_amount() {
            Some(v) => v,
            None => return,
        };
        println!("Added a new bill for {name} with amount - {amount}");
        bills.add(Bill { name, amount });
    }

    pub fn view_bill(bills: &Bills) {
        println!("Priting all bills:");
        for (name, amounts) in &bills.inner {
            println!("Bills for {name}");
            for amount in amounts {
                println!("{amount}");
            }
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(v) => v,
            None => return
        };
        if !bills.inner.contains_key(&name) {
            println!("Bill does not exist");
            return;
        }
        bills.inner.remove(&name);
        println!("Bills removed for user - {name}")
    }
    
    pub fn undo_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(v) => v,
            None => return,
        };
        if !bills.inner.contains_key(&name) {
            println!("Bill does not exist");
            return;
        }
        if let Some(amounts) = bills.inner.get_mut(&name) {
            amounts.pop();
            println!("Bills undone for user - {name}");
            if amounts.len() == 0 {
                bills.inner.remove(&name);
                println!("No Bills left for user - removed user entry");
            }
        }
    }
}

fn start() -> Option<String> {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(&input) {
            Some(MainMenu::AddEdit) => menu::add_or_edit_bill(&mut bills),
            Some(MainMenu::View) => menu::view_bill(&bills),
            Some(MainMenu::Remove) => menu::remove_bill(&mut bills),
            Some(MainMenu::Undo) => menu::undo_bill(&mut bills),
            _ => {
                println!("Invalid selection!!!");
                break
            }
        }
    }
    None
}

fn main() {
    start();
}