use std::io::BufRead;
use std::str::FromStr;

use crate::entities::expense::{Expense, ExpenseCategory, ExpenseModel, ExpenseType};

pub(crate) struct Display {
    expenses: Expense,
    main_menu: String,
    search_menu: String,
}

impl Display {
    pub fn new() -> Display {
        Display {
            expenses: Expense::new(),
            main_menu: String::from(
                "1. Add expense\n\
                2. Search expense\n\
                3. Update expense\n\
                4. Delete expense\n\
                5. Exit",
            ),
            search_menu: String::from(
                "1. Display every expense\n\
                2. Search by id\n\
                3. Search by date\n\
                4. Search by type\n\
                5. Search by category\n\
                6. Search by type and date\n\
                7. Back",
            ),
        }
    }
    pub fn print_main_menu(&self) {
        println!("{}", self.main_menu);
        print!("Enter number:\n");
    }
    pub fn print_search_menu(&self) {
        println!("{}", self.search_menu);
        print!("Enter number:\n");
    }
    pub fn add_new_expense(&mut self) {
        let new_expense = self.print_expense_form();
        self.expenses.append_expense(new_expense);
    }
    pub fn print_update_expense(&mut self) {
        println!("Enter id:");
        let id = self.get_int_user_response();
        let expense = self.print_expense_form();
        self.expenses.update_expense(expense, id);
    }
    pub fn print_delete_expense(&mut self) {
        println!("Enter id:");
        let id = self.get_int_user_response();
        self.expenses.delete_expense(id);
    }
    pub fn search_by_id(&self) {
        println!("Enter id:");
        let id = self.get_int_user_response();
        self.print_expense(&self.expenses.get_expense_by_id(id));
    }
    pub fn search_by_date(&self) {
        println!("Enter date:");
        let date = self.get_user_response();
        for expense in self.expenses.get_expenses_by_date(date) {
            self.print_expense(expense);
        }
    }
    pub fn search_by_category(&self) {
        self.print_expense_categories();
        println!("Enter category:");
        let expense_category = self.get_user_response();
        for expense in self
            .expenses
            .get_expenses_by_category(ExpenseCategory::from_str(expense_category.as_str()).unwrap())
        {
            self.print_expense(expense);
        }
    }
    pub fn search_by_type(&self) {
        self.print_expense_types();
        println!("Enter type:");
        let expense_type = self.get_user_response();
        for expense in self
            .expenses
            .get_expenses_by_type(ExpenseType::from_str(expense_type.as_str()).unwrap())
        {
            self.print_expense(expense);
        }
    }
    pub fn search_by_category_and_date(&self) {
        println!("Enter category:");
        let expense_category = self.get_user_response();
        println!("Enter date:\n");
        let date = self.get_user_response();
        for expense in self.expenses.get_expenses_by_category_and_date(
            ExpenseCategory::from_str(expense_category.as_str()).unwrap(),
            date,
        ) {
            self.print_expense(expense);
        }
    }
    pub fn print_all_expenses(&self) {
        for expense in self.expenses.get_all_expenses() {
            self.print_expense(expense);
        }
    }
    fn print_expense(&self, expense: &ExpenseModel) {
        println!(
            "Id: {}\nDate: {}\nType: {}\nCategory: {}\nAmount: {}\nDescription: {}",
            expense.id,
            expense.date,
            expense.expense_type,
            expense.expense_category,
            expense.amount,
            expense.description
        );
    }
    pub fn get_user_response(&self) -> String {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_line(&mut buffer).expect("Failed to read line");

        buffer.trim().to_string()
    }
    fn get_int_user_response(&self) -> i32 {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_line(&mut buffer).expect("Failed to read line");

        buffer.trim().parse::<i32>().unwrap()
    }
    fn print_expense_form(&self) -> ExpenseModel {
        let mut new_expense = ExpenseModel::default();
        println!("Enter date:");
        new_expense.date = self.get_user_response();
        self.print_expense_types();
        println!("Enter type:");
        new_expense.expense_type =
            ExpenseType::from_str(self.get_user_response().as_str()).unwrap();
        self.print_expense_categories();
        println!("Enter category:");
        new_expense.expense_category =
            ExpenseCategory::from_str(self.get_user_response().as_str()).unwrap();
        println!("Enter amount:");
        new_expense.amount = self.get_int_user_response();
        println!("Enter description:");
        new_expense.description = self.get_user_response();
        new_expense
    }
    fn print_expense_types(&self) {
        println!(
            "1. {}\n2. {}",
            ExpenseType::Essential,
            ExpenseType::NonEssential,
        );
    }
    fn print_expense_categories(&self) {
        println!(
            "1. {}\n2. {}\n3. {}\n4. {}",
            ExpenseCategory::Food,
            ExpenseCategory::Rent,
            ExpenseCategory::Transport,
            ExpenseCategory::Other,
        );
    }
    pub fn get_expenses(&self) -> &Expense {
        &self.expenses
    }
    pub fn set_expenses(&mut self, expenses: Expense) {
        self.expenses = expenses;
    }
}
