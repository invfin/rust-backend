mod expenses;
mod incomes;
mod investments;
mod transactions;

pub use transactions::{routes as transactions_routes, ApiDoc as ApiDocTransactions};
pub use expenses::{routes as expenses_routes, ApiDoc as ApiDocExpenses};
pub use incomes::{routes as incomes_routes, ApiDoc as ApiDocIncomes};
pub use investments::{routes as investments_routes, ApiDoc as ApiDocInvestments};
