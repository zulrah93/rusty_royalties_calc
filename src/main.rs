use std::fs::read_to_string;
use std::process::exit;
use std::env;
use std::collections::HashMap;
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
    let current_sale  = &mut Sale::new(0u64, 0.0f64);
    let mut song_title = "";

    let filtered_current_line : String = current_line.chars().filter(|&c| c != '"').collect();

    for column in filtered_current_line.as_str().split(',') {

        if column_index == 5 {
            song_title = column;
        }

        if column_index == 8 {

            if let Ok(streams_played) = column.parse::<u64>() {
                current_sale.streams_played += streams_played;
            }
            else {
                println!("Encountered an non integer in quanity column...");
                exit(0);
            }
        }

        if column_index == 13 {
            if let Ok(amount_usd) = column.parse::<f64>() {
                current_sale.amount_usd += amount_usd;
            }
        }

        column_index += 1;
    }

    (song_title.to_string(), *current_sale)
}

fn process_royalty(current_line : &str, results : &mut HashMap<String, Sale>) {

    let mut column_index : usize = 0;
    let mut current_sale  = &mut Sale::new(0u64, 0.0f64);

    let filtered_current_line : String = current_line.chars().filter(|&c| c != '"').collect();

    for column in filtered_current_line.as_str().split(',') {

        if column_index == 5 {
            let song_title = column.to_owned();
            current_sale  = results.entry(song_title).or_insert(Sale::new(0u64, 0.0f64));
        }

        if column_index == 8 {

            if let Ok(streams_played) = column.parse::<u64>() {
                   current_sale.streams_played += streams_played;
            }
            else {
                println!("Encountered an non integer in quanity column...");
                exit(0);
            }
        }

        if column_index == 13 {
            if let Ok(amount_usd) = column.parse::<f64>() {
                current_sale.amount_usd += amount_usd;
            }
        }

        column_index += 1;
    }
}

#[tokio::main]
async fn main() {

    let command_line_arguments: Vec<String> = env::args().collect();
    let mut rows_processed : usize  = 0;
    if command_line_arguments.len() > 1 {

        let mut use_async : bool = false;
        let csv_file_path = &command_line_arguments[1];
        if command_line_arguments.len() > 2 {
            use_async = command_line_arguments[2] == "async";
        }
        let mut results : HashMap<String, Sale> = HashMap::new();
        if let Ok(entire_file_in_memory) = read_to_string(csv_file_path) {

            if use_async {

                println!("Using async approach!");

                let mut futures = Vec::new();

                for current_line in entire_file_in_memory.lines() {
                    if rows_processed > 1 {
                       futures.push(process_royalty_async(current_line));
                    }
                    rows_processed += 1;
                }

                for future in futures {
                   let new_sale = future.await;
                   let sale = results.entry(new_sale.0).or_insert(Sale::new(0u64, 0f64));
                    sale.amount_usd += new_sale.1.amount_usd;
                    sale.streams_played += new_sale.1.streams_played;
                }

                for key_value in results {
                    println!("=======================================================\nSong Title: {}", key_value.0);
                    println!("{}", key_value.1);
                }

                println!("Processed {} rows...", rows_processed);
                exit(0);
            }

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
