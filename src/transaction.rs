// transaction.rs

use crate::location::{Country, Continent};
use chrono::NaiveDate;
use std::str::FromStr;

#[derive(Debug)]
pub struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    transaction_start_date: NaiveDate,
    transaction_end_date: NaiveDate,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,
}

impl Transaction {
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 7 {
            return Err(format!("Expected 7 fields but found {}", fields.len()));
        }

        let transaction_id = fields[0].parse::<u32>()
            .map_err(|e| format!("Error parsing transaction_id: {}", e))?;
        let client_id = fields[1].parse::<u32>()
            .map_err(|e| format!("Error parsing client_id: {}", e))?;
        let asset_name = fields[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d")
            .map_err(|e| format!("Error parsing transaction_start_date: {}", e))?;
        let transaction_end_date = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d")
            .map_err(|e| format!("Error parsing transaction_end_date: {}", e))?;
        let country = fields[5].parse::<Country>()
            .map_err(|e| format!("Error parsing country: {}", e))?;
        let amount = fields[6].parse::<f64>()
            .map_err(|e| format!("Error parsing amount: {}", e))?;
        let days_under_management = transaction_end_date.signed_duration_since(transaction_start_date).num_days();
        let continent = country.country_to_continent();

        Ok(Transaction {
            transaction_id,
            client_id,
            asset_name,
            transaction_start_date,
            transaction_end_date,
            country,
            continent,
            amount,
            days_under_management,
        })
    }

    pub fn get_continent(&self) -> &Continent {
        &self.continent
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}
