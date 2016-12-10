
use std::io::{self, BufRead};
use std::collections::LinkedList;

struct AddressBook {
    id: u32,
    name: String,
    address: String,
    postcode: String,
    city: String,
    country: String
}

fn main() {
    let reader = io::stdin();
    let mut command;
    let mut exitProgram = false;

    let mut addr_list: LinkedList<AddressBook> = load_existing_addressbook();

    println!("Welcome to the Address Book Manager. You have the following options at your disposal:");
    println!("1. Create/Add address");
    println!("2. Modify existing address");
    println!("3. Delete existing address");
    println!("4. Show current addresses");
    println!("5. Exit/Quit");

    for line in reader.lock().lines()   {

        command = line.unwrap();

        match &*command.to_lowercase() {
            "1"|"add"|"create"  => {
                ;
            }
            "2"|"modify"        => {
                ;
            }
            "3"|"delete"        => {
                ;
            }
            "4"|"show"|"current"=> {
                ;
            }
            "5"|"quit"|"exit"   => {
                exitProgram = true;
                break;
            }
            _               => {
                println!("The command '{}' was not found.", &*command.to_lowercase());
            }
        }
        command.clear();
    }
}

fn load_existing_addressbook() -> LinkedList<AddressBook> {
    return LinkedList::new();
}

fn create_new_address() {

}

fn modify_address(id: u32) {

}

fn delete_address(id: u32) {

}
