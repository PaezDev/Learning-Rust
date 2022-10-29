#![allow(unused)]
use std::{
    io::{
        self,
        Write,
    },
    process,
};

use std::collections::HashMap;

fn get_median(vec: &Vec<i32>) -> i32 { 
    if vec.len() == 0 {
        println!("No values found in list...\n");
        return -1;
    }
    let middle_val = vec.len() / 2; 
    vec[middle_val] 
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    if vec.len() == 0 {
        println!("No values found in list...\n");
        return -1;
    }

    let mut mode = HashMap::new();

    for &item in vec {
        *mode.entry(item).or_insert(0) += 1
    }

    mode 
        .into_iter() 
        .max_by_key(|&(_, count) | count)
        .map(|(val, _) | val)
        .expect("Cannot compute the mode of zero numbers")
}

// prints current list
fn print_list(vec: &Vec<i32>) {
    if vec.len() == 0 {
        println!("No Values...\n");
    } else {
        println!("Printing Sorted List...\n");
        for i in vec.iter() {
            print!("|{i}|");
        }
        println!("\n");
    }
}

// ask user how many entries they wish to input
// push into vec until end of count.
fn create_list(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    print!("Amount of values? ");

    let mut count = get_val(); 
    println!();

    if count > 20 {
        println!("Do you like pain? 1 | 0 ");
        let choice = get_val();
        if choice == 0 {
            println!();
            return vec;
        }
    }

    let mut i = 1;

    while count != 0 {
        print!("{i} >> ");
        let val = get_val();
        vec.push(val);
        count -= 1;
        i += 1;
    }
    vec.sort();
    vec
}

// get menu, count, and input vals
fn get_val() -> i32 {
    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let count = input
        .trim()
        .parse::<i32>()
        .unwrap_or_else(|_| {
            eprintln!("Entered input is not valid");                     
            drop(input);
            process::exit(1);
        });
    count

}

fn basic_menu() {
    println!("\t\tMedian & Mode");
    println!("---------------------------------------------");
    println!("1. Create/Add Integer List");
    println!("2. Reset Integer List");
    println!("3. Print Current List");
    println!("4. Find Median");
    println!("5. Find Mode");
    println!("6. Quit");
    println!("---------------------------------------------");
    print!("M&M >> ");
}

fn main() {
    let mut vec = Vec::new();

    loop {
        basic_menu();

        let choice = get_val();
        println!();

        match choice {
            1 => {
                create_list(&mut vec);
            },
            2 => main(),
            3 => print_list(&vec),
            4 => {
                println!("Getting Median...\n");
                println!(">> |{}|", get_median(&vec));
            },
            5 => {
                println!("Getting Mode...\n");
                println!(">> |{}|", get_mode(&vec))
            }
            6 => {
                println!("Goodbye!");
                process::exit(0);
            },
            _ => {
                println!("Invalid input...");
                process::exit(1);
            },
        };
    }
}
