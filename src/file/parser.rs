use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::str::FromStr;

use crate::entities::expense::{Expense, ExpenseCategory, ExpenseModel, ExpenseType};

pub struct FileData {
    name: &'static str,
}

impl FileData {
    pub fn new(name: &'static str) -> FileData
    where
        &'static str: AsRef<Path>,
    {
        FileData { name }
    }
    pub fn save_date(&self, expense: &Expense) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.name)
            .expect("Unable to open file");
        for expense_model in expense.get_all_expenses() {
            let line = format!(
                "{};{};{};{};{};{}\n",
                expense_model.id,
                expense_model.date,
                expense_model.expense_category.to_string(),
                expense_model.expense_type.to_string(),
                expense_model.amount,
                expense_model.description
            );
            file.write(line.as_bytes()).expect("Unable to write data");
        }
    }
    pub fn get_parsed_data(&mut self) -> Expense {
        let mut expense = Expense::new();
        if let Ok(lines) = self.read_file() {
            for line in lines.flatten() {
                let mut expense_model = ExpenseModel::default();
                let line: Vec<&str> = line.split(";").collect();
                expense_model.id = line[0].parse::<i32>().unwrap();
                expense_model.date = line[1].into();
                expense_model.expense_category = ExpenseCategory::from_str(line[2]).unwrap();
                expense_model.expense_type = ExpenseType::from_str(line[3]).unwrap();
                expense_model.amount = line[4].parse::<i32>().unwrap();
                expense_model.description = line[5].into();
                expense.append_expense(expense_model);
            }
        } else {
            File::create("expenses.txt").expect("Unable to create file");
        }
        expense
    }
    fn read_file(&mut self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.name)?;
        Ok(io::BufReader::new(file).lines())
    }
}
