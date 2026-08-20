mod clients;
mod finance;
mod project_states;
mod sales;
mod teams;

pub use finance::get_financial_status;
pub use sales::{get_sales_metrics, get_top_products};
