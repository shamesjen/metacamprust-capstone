use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use transaction::Transaction;
use location::Continent;

mod transaction;
mod location;

fn filter_by_continent<'a>(transactions: &'a [Transaction], continent: &Continent) -> Vec<&'a Transaction> {
    transactions.iter().filter(|&t| *t.get_continent() == *continent).collect()
}

fn main() {
    // Open the CSV file
    let file = File::open("./transactions.csv").unwrap();
    let reader = BufReader::new(file);

    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines: Vec<(usize, String, String)> = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }

        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(error) => skipped_lines.push((idx, error, line_str.clone())),
        }
    }

    for transaction in &transactions {
        println!("{:?}", transaction);
    }

    for (idx, error, line) in &skipped_lines {
        println!("Line {}: Error: {}. Content: {}", idx, error, line);
    }

    // Utilize HashMap to keep track of the total invested amount per continent
    let mut continent_investment: HashMap<String, f64> = HashMap::new();

    continent_investment = transactions.iter().fold(continent_investment, |mut acc, transaction| {
        let continent_str = format!("{:?}", transaction.get_continent());
        *acc.entry(continent_str).or_insert(0.0) += transaction.get_amount();
        acc
    });
    
    // Print total investment per continent
    continent_investment.iter().for_each(|(continent, total)| {
        println!("Total investment in {}: {}", continent, total);
    });
    
    // Filter and print only transactions with European companies using iter and filter
    transactions.iter()
        .filter(|&transaction| *transaction.get_continent() == Continent::Europe)
        .for_each(|transaction| {
            println!("{:?}", transaction);
        });
}