/* */

use clap::Parser;
use friend::Friend;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{fs::OpenOptions, path::PathBuf};
mod friend;

const FILE_HEADER: &str = "Firstname,Surname,Phonenumber";
const INIT_SIZE: usize = 5;
const THRESHOLD_FOR_RESIZE: f64 = 0.8;
#[derive(Parser)]
struct Args {
    path: PathBuf,
}
fn main() {
    let args = Args::parse();
    let (mut friends, mut size_of_table) = get_csv_content(&args.path);

    let input = std::io::stdin().lock().lines();
    println!("Your inside the application now. Use exit to exit and save.");
    for _temp in input {
        let line = _temp.unwrap();
        let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
        if split_line.len() == 0 {
            print_help();
            continue;
        }
        match split_line[0].to_ascii_lowercase().trim() {
            "add" => {
                if split_line.len() == 4 {
                    //quite ugly, but i cant be bothered to make add_to_table much more neat for CLI inpit.
                    let pass_line = format!(
                        "{},{},{}",
                        split_line[1].trim(),
                        split_line[2].trim(),
                        split_line[3].trim()
                    );
                    size_of_table +=
                        add_to_table(&mut friends, size_of_table, Friend::from_line(pass_line));
                    println!("Added the friend to the database, if it didnt already exist.");
                } else {
                    print_help();
                }
            }
            "remove" => {
                if split_line.len() == 3 {
                    size_of_table -= remove(&mut friends, (split_line[1].to_owned() + " " + split_line[2]).as_str());
                    println!("Removed friend, if you were friends.");
                } else {
                    print_help();
                }
            },
            "listall" => listall(&friends),
            "struct" => print_hash_map(&friends),
            "number" => {
                if split_line.len() == 3 {
                get_value(&friends, (split_line[1].to_owned() + " " + split_line[2]).as_str());
                } else {
                    print_help();
                }
            }

            "exit" => break,

            _ => print_help(),
        };
        println!("\nYour inside the application now. Use exit to exit and save.");
    }

    save_to_csv(args.path, &friends);
}

fn print_help() {
    println!("Unkown command\n\nCommand list:\n--------------");
    println!(">add <firstname> <surname> <phonenumber>");
    println!(">remove <firstname> <surname>");
    println!(">number <firstname> <surname>");
    println!(">listall");
    println!(">struct");
    println!(">exit");
}

/// Generates a Vec<Vec<Friend>> and a usize.
/// the vector is the table, and the usize is the amount of entries.
#[allow(unused_must_use)]
fn get_csv_content(path: &PathBuf) -> (Vec<Vec<Friend>>, usize) {
    let open = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path);
    let file = match open {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("File not found. Do you want to create the file? (y/n): ");
                let mut choice = String::new();
                std::io::stdin().lock().read_line(&mut choice);
                if choice.trim().to_lowercase() == "y" {
                    return {
                        let mut _vec: Vec<Vec<Friend>> = Vec::with_capacity(INIT_SIZE);
                        for i in 0..INIT_SIZE {
                            _vec.push(Vec::new());
                        }
                        (_vec, 0)
                    };
                } else {
                    panic!("No operation was done. File not found and not created.");
                }
            }
            ErrorKind::PermissionDenied => {
                panic!("Permission denied. Try running in admin mode maybe idk.");
            }
            _ => {
                panic!("Error: {}", err);
            }
        },
    };
    let mut lines = BufReader::new(file).lines().map(|x| x.unwrap());
    if !(lines.next().unwrap_or(String::from("")) == FILE_HEADER) {
        panic!("The file does not follow the database format");
    }
    /*
    //GG it cant guess so this is always 1
    let amount_of_lines = {
        let _temp = lines.size_hint();
        match _temp.1 {
            Some(value) => value,
            None => _temp.0 + 1, //plus one because why not, nah but fr if its 0 i think its better its 1. and adding 1 isnt bad
        }
    };*/
    let mut table: Vec<Vec<Friend>> = Vec::with_capacity(INIT_SIZE);
    //table.fill(Vec::new()); <-- This is cursed, doesnt actually fill
    for _ in 0..INIT_SIZE {
        table.push(Vec::new())
    }
    let mut size_of_table: usize = 0;
    for line in lines {
        let friend = Friend::from_line(line);
        size_of_table += add_to_table(&mut table, size_of_table, friend);
    }

    (table, size_of_table)
}

fn hash(name: &str, _capacity: usize) -> usize {
    let mut hash = 0;
    for (index, char) in name.chars().enumerate() {
        hash += hash * index + char as usize;
    }
    hash % _capacity
}

fn resize(table: &mut Vec<Vec<Friend>>) {
    let _new_cap = table.capacity() * 2;
    let mut _vec: Vec<Vec<Friend>> = Vec::with_capacity(_new_cap);
    for _ in 0.._new_cap {
        _vec.push(Vec::new());
    }
    for outer in 0..table.len() {
        while true {
            if let Some(friend) = table[outer].pop() {
                //its a bit cursed but we are fine saying size = 0, since it's only used for purpose of resizing
                //and we do know that we wont exceed capacity, since we just doubled it. 
                add_to_table(&mut _vec, 0, friend);
            }
            else {
                break;
            }
        }
    }
    *table = _vec;
}

//returns 1 if added succesfully, returns 0 if not adding
fn add_to_table(table: &mut Vec<Vec<Friend>>, current_size: usize, friend: Friend) -> usize {
    if (current_size + 1) as f64 / table.capacity() as f64 > THRESHOLD_FOR_RESIZE {
        //resize
        resize(table);

    }
    //hash from combination of firstname + surname
    let mut name = friend.firstname.clone();
    name += " ";
    name += &friend.surname;

    let hash_index = hash(&name, table.capacity());
    
    for _fr in table[hash_index].iter_mut() {
        if _fr.firstname == friend.firstname && _fr.surname == friend.surname  {
            println!("Change phonenumber");
            _fr.phonenumber = friend.phonenumber;
            return 0;
        }
    }
    table[hash_index].push(friend);
    1
}

// returns 1 if it removed an element. else 0
fn remove(table: &mut Vec<Vec<Friend>>, name: &str) -> usize{
    let hash_index = hash(name, table.capacity());

    let (_first, _second) = name.split_once(" ").unwrap();
    for index in 0..table[hash_index].len() {
        if (_first == table[hash_index][index].firstname) && (_second == table[hash_index][index].surname) {
            table[hash_index].swap_remove(index);
            return 1;
        }
    }
    0
}

fn get_value(table: &Vec<Vec<Friend>>, name: &str) {
    let hash_index = hash(name, table.capacity());
    let (_first, _second) = name.split_once(" ").unwrap();
    for friend in table[hash_index].iter() {
        if (_first == friend.firstname) && (_second == friend.surname) {
            println!("{} has phonenumber {}", name, friend.phonenumber);

        }
    }
    
}

fn listall(table: &Vec<Vec<Friend>>) {
    println!("This is your friend list:\n**************************\n**************************");
    for _vec in table {
        for friend in _vec {
            println!("Friend: {} {} with phonenumber {}", friend.firstname, friend.surname, friend.phonenumber);
        }
    }
    println!("You have no more friends!")
}

#[allow(unused)]
fn save_to_csv(path: PathBuf, table: &Vec<Vec<Friend>>) {
    let open = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .ok();
    if let Some(mut file) = open {
        file.write_all(FILE_HEADER.as_bytes());
        for vec in table {
            for friend in vec {
                file.write_all(b"\n");
                file.write_all(friend.get_line().as_bytes());
            }
        }
    } else {
        //some error happened try to save it where we are.
        save_fail(0, table);
    }
}

#[allow(unused)]
fn save_fail(attempt: usize, table: &Vec<Vec<Friend>>) {
    if attempt > 100 {
        panic!("just ff, it cant fucking be saved, all data lost apperently");
    }

    let open = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("table{}", attempt))
        .ok();
    if let Some(mut file) = open {
        file.write_all(FILE_HEADER.as_bytes());
        for vec in table {
            for friend in vec {
                file.write_all(b"\n");
                file.write_all(friend.get_line().as_bytes());
            }
        }
    } else {
        //some error happened try to save it where we are.
        save_fail(attempt + 1, table);
    }
}


#[test]
fn test() {

    /*THIS IS SO CURSED
    WHY DOESNT RUST FILL MY VECTORS!!
    */
    let mut _vec: Vec<Vec<usize>> = Vec::with_capacity(10);
_vec.fill(Vec::new());
assert!(_vec.len() == 0);

let mut _v: Vec<usize> = Vec::with_capacity(10);
_v.fill(0);
assert!(_v.len() == 0);
}

fn print_hash_map(table: &Vec<Vec<Friend>>) {
    println!("The map looks like:\n");
    for _vec in table.iter().enumerate() {
        println!("Slot{}", _vec.0);
        for friend in _vec.1 {
            println!("\t{} {}", friend.firstname, friend.surname);
        }
    }
}