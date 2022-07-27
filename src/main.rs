/* 
 * This is a cook book implementation
 * That keeps track of recipes that you have
 * and all ingredients that you own.
 * * You can add/remove ingredients and recipes.
 * * You can "make" a recipe and consume some of the ingredients,
 * * also you can see which ingredients you are missing in order to make a recipe
 */
use crate::menu::*;
pub mod menu;

fn main() {
    let menu_options = vec![
        MenuOption::new(1, "List all inhredients"),
        MenuOption::new(2, "Add new ingredient"),
        MenuOption::new(3, "Remove ingredient"),
        MenuOption::new(4, "Quit"),
    ];

    let mut user_choice;

    loop {
        user_choice = menu(&menu_options);
        match user_choice {
            1 => println!("Going to print all ingredients!"),
            2 => println!("Lets add a new inhredient!"),
            3 => println!("Sadly we have to remove one ingredient!"),
            4 => { 
                println!("Quitin!");
                break;
            },
            _ => ()
        };
    }

}
