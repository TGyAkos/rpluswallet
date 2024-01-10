use std::convert::Into;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(PartialEq)]
pub(crate) enum ExpenseCategory {
    Food,
    Rent,
    Transport,
    Other,
}
impl FromStr for ExpenseCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Food"  => Ok(ExpenseCategory::Food),
            "Rent"  => Ok(ExpenseCategory::Rent),
            "Transport" => Ok(ExpenseCategory::Transport),
            "Other" => Ok(ExpenseCategory::Other),
            _      => Err("Invalid expense category".into()),
        }
    }
}
impl fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExpenseCategory::Food => write!(f, "Food"),
            ExpenseCategory::Rent => write!(f, "Rent"),
            ExpenseCategory::Transport => write!(f, "Transport"),
            ExpenseCategory::Other => write!(f, "Other"),
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum ExpenseType {
    Essential,
    NonEssential,
}
impl FromStr for ExpenseType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Essential"  => Ok(ExpenseType::Essential),
            "NonEssential"  => Ok(ExpenseType::NonEssential),
            _      => Err("Invalid expense type".into()),
        }
    }
}
impl fmt::Display for ExpenseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExpenseType::Essential => write!(f, "Essential"),
            ExpenseType::NonEssential => write!(f, "NonEssential"),
        }
    }
}

pub(crate) struct ExpenseModel {
    pub(crate) id: i32,
    pub(crate) date: String,
    pub(crate) expense_type: ExpenseType,
    pub(crate) expense_category: ExpenseCategory,
    pub(crate) amount: i32,
    pub(crate) description: String,
}
impl Default for ExpenseModel {
    fn default() -> ExpenseModel {
        ExpenseModel {
            id: 0,
            date: String::new(),
            expense_type: ExpenseType::Essential,
            expense_category: ExpenseCategory::Food,
            amount: 0,
            description: String::new(),
        }
    }
}

pub(crate) struct Expense {
    all_expenses: Vec<ExpenseModel>,
    biggest_id: i32,
}

impl Expense {
    pub fn new() -> Expense {
        Expense {
            all_expenses: Vec::new(),
            biggest_id: 1,
        }
    }
    pub fn append_expense(&mut self, mut expense_model: ExpenseModel) {
        expense_model.id = self.biggest_id;
        if self.biggest_id <= expense_model.id { self.biggest_id += 1 };
        self.all_expenses.push(expense_model);
        self.sort_expenses_by_id();
    }
    pub fn create_new_expense(&mut self,
                          date: String,
                          expense_type: ExpenseType,
                          expense_category: ExpenseCategory,
                          amount: i32,
                          description: String
                          ) -> ExpenseModel {
        let id: i32 = self.biggest_id;
        self.biggest_id += 1;
        ExpenseModel {
            id,
            date,
            expense_type,
            expense_category,
            amount,
            description,
        }
    }
    pub fn sort_expenses_by_id(&mut self) {
        self.all_expenses.sort_by(|a, b| a.id.cmp(&b.id));
    }
    pub fn update_expense(&mut self, expense_model: ExpenseModel, id: i32) {
        self.all_expenses[id as usize] = expense_model;
    }
    pub fn delete_expense(&mut self, id: i32) {
        self.all_expenses.remove(id as usize - 1);
    }
    pub fn get_all_expenses(&self) -> &Vec<ExpenseModel> {
        &self.all_expenses
    }
    pub fn get_expense_by_id(&self, id: i32) -> &ExpenseModel {
        &self.all_expenses[id as usize]
    }
    pub fn get_expenses_by_category(&self, category: ExpenseCategory) -> Vec<&ExpenseModel> {
        let mut expenses: Vec<&ExpenseModel> = Vec::new();
        for expense in &self.all_expenses {
            if expense.expense_category == category {
                expenses.push(expense);
            }
        }
        expenses
    }
    pub fn get_expenses_by_type(&self, expense_type: ExpenseType) -> Vec<&ExpenseModel> {
        let mut expenses: Vec<&ExpenseModel> = Vec::new();
        for expense in &self.all_expenses {
            if expense.expense_type == expense_type {
                expenses.push(expense);
            }
        }
        expenses
    }
    pub fn get_expenses_by_date(&self, date: String) -> Vec<&ExpenseModel> {
        let mut expenses: Vec<&ExpenseModel> = Vec::new();
        for expense in &self.all_expenses {
            if expense.date == date {
                expenses.push(expense);
            }
        }
        expenses
    }
    pub fn get_expenses_by_category_and_date(&self, expense_category: ExpenseCategory, date: String) -> Vec<&ExpenseModel> {
        let mut expenses: Vec<&ExpenseModel> = Vec::new();
        for expense in &self.all_expenses {
            if expense.expense_category == expense_category && expense.date == date {
                expenses.push(expense);
            }
        }
        expenses
    }
    pub fn set_expenses(&mut self, expenses: Vec<ExpenseModel>) {
        self.all_expenses = expenses;
        self.sort_expenses_by_id();
    }
}