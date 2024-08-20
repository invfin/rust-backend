use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use futures_util::{stream::FuturesUnordered, StreamExt};
use polars::{
    datatypes::AnyValue,
    io::SerReader,
    lazy::dsl::{col, lit, StrptimeOptions},
    prelude::{IntoLazy, LazyCsvReader, LazyFileListReader},
};

#[derive(Debug)]
struct TransactionDetail {
    date: NaiveDate,
    description: Option<String>,
    comment: Option<String>,
    file_id: i64,
    currency_id: i64,
    investment_details_id: Option<i64>,
    original_amount: BigDecimal,
}

#[derive(Debug)]
struct Transaction {
    user_id: i64,
    details_id: i64,
    exchange_rate_id: Option<i64>,
    date: NaiveDate,
    amount: BigDecimal,
    category: String,
}

#[derive(Debug)]
pub struct TransactionWrapper {
    transaction: Transaction,
    details: TransactionDetail,
}

trait TransactionsManager {
    async fn to_transaction_wraper(self) -> TransactionWrapper;
}

const FIRSTTRADE_COLUMNS_NEW: [&str; 13] = [
    "symbol",
    "quantity",
    "price",
    "action",
    "description",
    "trade_date",
    "settled_date",
    "interest",
    "amount",
    "commission",
    "fee",
    "cusip",
    "record_type",
];

const FIRSTTRADE_COLUMNS_ORIGINAL: [&str; 13] = [
    "Symbol",
    "Quantity",
    "Price",
    "Action",
    "Description",
    "TradeDate",
    "SettledDate",
    "Interest",
    "Amount",
    "Commission",
    "Fee",
    "CUSIP",
    "RecordType",
];

struct FirstradeRow {
    symbol: String,
    quantity: f64,
    price: f64,
    action: String,
    description: String,
    trade_date: i64,
    settled_date: i64,
    interest: f64,
    amount: f64,
    commission: f64,
    fee: f64,
}

impl TransactionsManager for FirstradeRow {
    async fn to_transaction_wraper(self) -> TransactionWrapper {
        TransactionWrapper {
            transaction: todo!(),
            details: todo!(),
        }
    }
}

pub async fn parse_firstrade(user_id: i64, path: &str) -> Vec<TransactionWrapper> {
    FuturesUnordered::from_iter(
        LazyCsvReader::new(path)
            .with_has_header(true)
            .with_rechunk(true)
            .finish()
            .unwrap()
            .rename(FIRSTTRADE_COLUMNS_ORIGINAL, FIRSTTRADE_COLUMNS_NEW)
            .with_columns(vec![
                col("symbol").str().strip_chars(lit(false)),
                col("trade_date").str().to_date(StrptimeOptions::default()),
                col("settled_date")
                    .str()
                    .to_date(StrptimeOptions::default()),
            ])
            .collect()
            .unwrap()
            .into_struct("StructChunked")
            .iter()
            .map(|row| {
                FirstradeRow {
                    symbol: row[0].get_str().unwrap().to_string(),
                    quantity: row[1].try_extract().unwrap(),
                    price: row[2].try_extract().unwrap(),
                    action: row[3].get_str().unwrap().to_string(),
                    description: row[4].get_str().unwrap().to_string(),
                    trade_date: row[5].try_extract().unwrap(),
                    settled_date: row[6].try_extract().unwrap(),
                    interest: row[7].try_extract().unwrap(),
                    amount: row[8].try_extract().unwrap(),
                    commission: row[9].try_extract().unwrap(),
                    fee: row[10].try_extract().unwrap(),
                }
                .to_transaction_wraper()
            }),
    )
    .collect()
    .await
}

struct CreditAgricoleRow {
    date: NaiveDate,
    description: String,
    debit: Option<BigDecimal>,
    credit: Option<BigDecimal>,
}

impl CreditAgricoleRow {
    fn new(date: &str, description: &str, debit: &str, credit: &str) -> Self {
        Self {
            date: NaiveDate::from_str(date).unwrap(),
            description: description.to_owned(),
            debit: BigDecimal::from_str(debit).ok(),
            credit: BigDecimal::from_str(credit).ok(),
        }
    }
}

impl TransactionsManager for CreditAgricoleRow {
    async fn to_transaction_wraper(self) -> TransactionWrapper {
        TransactionWrapper {
            transaction: todo!(),
            details: todo!(),
        }
    }
}

pub async fn parse_credit_agricole(user_id: i64, path: &str) -> Vec<TransactionWrapper> {
    FuturesUnordered::from_iter(
        LazyCsvReader::new(path)
            .with_has_header(true)
            .with_rechunk(true)
            .finish()
            .unwrap()
            .with_columns(vec![col("Date").str().to_date(StrptimeOptions::default())])
            .collect()
            .unwrap()
            .into_struct("StructChunked")
            .iter()
            .map(|row| {
                CreditAgricoleRow::new(
                    row[0].get_str().unwrap(),
                    row[1].get_str().unwrap(),
                    row[2].get_str().unwrap(),
                    row[3].get_str().unwrap(),
                )
                .to_transaction_wraper()
            }),
    )
    .collect()
    .await
}
