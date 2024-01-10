mod file;
mod entities;
mod console;

use crate::entities::expense::Expense;
use crate::console::display::Display;
use crate::file::parser::FileData;
fn main() {
    let mut app = App::new();
    let parsed_data = app.parser.get_parsed_data();
    app.set_display_expenses(parsed_data);
    app.main_loop();
    app.parser.save_date(app.get_display_expenses());
}

struct App {
    display: Display,
    parser: FileData,
}
impl App {
    fn new() -> App {
        App {
            display: Display::new(),
            parser: FileData::new("./expenses.txt"),
        }
    }
    fn main_loop(&mut self) {
        loop {
            self.display.print_main_menu();
            let user_response = self.display.get_user_response();
            match user_response.as_str().trim() {
                "1" => self.display.add_new_expense(),
                "2" => self.search_loop(),
                "3" => self.display.print_update_expense(),
                "4" => self.display.print_delete_expense(),
                "5" => break,
                _ => println!("Invalid option"),
            }
        }
    }
    fn search_loop(&self) {
        loop {
            self.display.print_search_menu();
            let user_response = self.display.get_user_response();
            match user_response.as_str().trim() {
                "1" => self.display.print_all_expenses(),
                "2" => self.display.search_by_id(),
                "3" => self.display.search_by_date(),
                "4" => self.display.search_by_category(),
                "5" => self.display.search_by_type(),
                "6" => self.display.search_by_category_and_date(),
                "7" => break,
                _ => println!("Invalid option"),
            }
        }
    }
    fn get_display_expenses(&self) -> &Expense {
        &self.display.get_expenses()
    }
    fn set_display_expenses(&mut self, expenses: Expense) {
        self.display.set_expenses(expenses);
    }
}



