use std::io::stdin;

#[derive(Debug)]
pub struct MenuOption<'a> {
    id: u32,
    text: &'a str,
}
impl<'a> MenuOption<'a> {
    pub fn new(id: u32, text: &'a str) -> MenuOption {
        MenuOption {
            id,
            text,
        }
    }
}
pub fn menu(menu_options: &Vec<MenuOption>) -> u32 {
    // Print all options
    
    for option in menu_options {
        println!("({}) {}", option.id, option.text);
    }

    // Get user input
    let mut user_input;
    let mut parsed_user_input: u32 = 0;

    while parsed_user_input == 0 {

        user_input = String::from("");

        println!("Please choose an option");
        match stdin().read_line(&mut user_input) {
            Err(_) => continue,
            _ => ()
        }

        parsed_user_input = match user_input.trim().parse() {
            Ok(n) => n,
            _ => { 
                println!("Invalid input. Please enter a non negative number");
                0
            },
        }

    };

    parsed_user_input
}
