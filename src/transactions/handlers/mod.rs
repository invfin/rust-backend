mod expenses;
mod incomes;
mod investments;

pub use expenses::{routes as expenses_routes, ApiDoc as ApiDocExpenses};
pub use incomes::{routes as incomes_routes, ApiDoc as ApiDocIncomes};
pub use investments::{routes as investments_routes, ApiDoc as ApiDocInvestments};
