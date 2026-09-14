use std::fs::read_to_string;
use std::process::exit;
use std::env;
use std::collections::HashMap;
//use std::cell::RefCell;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
struct Sale {
    streams_played : u64,
    amount_usd : f64
}

impl Display for Sale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "=======================================================\nStreams Played: {}\nAmount Earned in USD: {}",
                    self.streams_played, self.amount_usd)
    }
}

impl Sale {

    fn new(streams_played : u64, amount_usd : f64) -> Self {
        Sale{
            streams_played,
            amount_usd
        }
    }

}
async fn process_royalty_async(current_line : &str) -> (String, Sale) {

    let mut column_index : usize = 0;
    let mut current_sale  = &mut Sale::new(0u64, 0.0f64);
    let mut song_title = String::new();

    for column in current_line.split(',') {

        let true_column : String = column.chars().filter(|&c| c != '"').collect();

        if column_index == 5 {
            song_title = true_column.to_owned();
        }

        if column_index == 8 {

            if let Ok(streams_played) = true_column.parse::<u64>() {
                current_sale.streams_played += streams_played;
            }
            else {
                println!("Encountered an non integer in quanity column...");
                exit(0);
            }
        }

        if column_index == 13 {
            if let Ok(amount_usd) = true_column.parse::<f64>() {
                current_sale.amount_usd += amount_usd;
            }
        }

        column_index += 1;
    }

    (song_title, *current_sale)
}

fn process_royalty(current_line : &str, results : &mut HashMap<String, Sale>) {

    let mut column_index : usize = 0;
    let mut current_sale  = &mut Sale::new(0u64, 0.0f64);
    for column in current_line.split(',') {

        let true_column : String = column.chars().filter(|&c| c != '"').collect();

        if column_index == 5 {
            let song_title = true_column.to_owned();
            current_sale  = results.entry(song_title).or_insert(Sale::new(0u64, 0.0f64));
        }

        if column_index == 8 {

            if let Ok(streams_played) = true_column.parse::<u64>() {
                   current_sale.streams_played += streams_played;
            }
            else {
                println!("Encountered an non integer in quanity column...");
                exit(0);
            }
        }

        if column_index == 13 {
            if let Ok(amount_usd) = true_column.parse::<f64>() {
                current_sale.amount_usd += amount_usd;
            }
        }

        column_index += 1;
    }
}

fn main() {

    let command_line_arguments: Vec<String> = env::args().collect();
    let mut rows_processed = 0;
    if command_line_arguments.len() > 1 {

        let csv_file_path = &command_line_arguments[1];
        let mut results : HashMap<String, Sale> = HashMap::new();

        if let Ok(entire_file_in_memory) = read_to_string(csv_file_path) {

            for current_line in entire_file_in_memory.lines() {
                if rows_processed > 1 {
                    process_royalty(current_line, &mut results);
                }
                rows_processed += 1;
            }

            for key_value in results {
                    println!("=======================================================\nSong Title: {}", key_value.0);
                    println!("{}", key_value.1);
            }

            println!("Processed {} rows...", rows_processed);
            exit(0);
        }
        else {
            println!("Failed to load csv into memory...");
            exit(0);
        }
    }

    println!("No csv file path with royalties supplied program will exit...");
}
