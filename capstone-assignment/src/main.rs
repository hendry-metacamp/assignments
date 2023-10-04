/// Location Enums
mod location;

/// Transaction functions
mod transaction;

mod utils;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use transaction::Transaction;

fn main() {
    let file = File::open("./transactions.csv").unwrap();
    let reader = BufReader::new(file);
    let mut transactions: Vec<Transaction> = Vec::new();

    let mut skipped_lines: Vec<_> = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let line_str = line.unwrap();

        let parsed_transaction = Transaction::from_csv_line(&line_str);
        match parsed_transaction {
            Ok(t) => transactions.push(t),
            Err(error_string) => skipped_lines.push((idx, error_string, line_str)),
        }
    } //end of for loop

    for transaction in &transactions {
        println!("Valid: {:?}", transaction);
    }

    for skipped_line in &skipped_lines {
        println!("Skipped: {:?}", skipped_line);
    }

    // Check investments by continents
    utils::investments_by_continents(&transactions);

    // List all transactions in Europe
    utils::list_by_continent(&transactions, &location::Continent::Europe);

    // List transactions 1..5 in Europe for the first 5
    utils::list_by_continent(&transactions[1..5], &location::Continent::Europe);

    // List all transactions in Asia
    utils::list_by_continent(&transactions, &location::Continent::Asia);

    // List transactions 3..20 in Asia
    utils::list_by_continent(&transactions[3..20], &location::Continent::Asia);
}
