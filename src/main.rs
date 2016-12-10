
//use std::io::{self, BufRead, Write, BufWriter};
use std::io::{self, BufRead};
use std::collections::LinkedList;
//use std::fs::File;

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

    // load the list of existing addresses into the List of AddresBook.
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
                save_address_list(&addr_list);
                break;
            }
            _               => {
                println!("The command '{}' was not found.", &*command.to_lowercase());
            }
        }
        command.clear();
    }
}

// Loads the addressbook from the current directory into a Linked List.
fn load_existing_addressbook() -> LinkedList<AddressBook> {
    return LinkedList::new();
}

fn create_new_address(curr_list: &LinkedList<AddressBook>) {

}

fn modify_address(curr_list: &LinkedList<AddressBook>, id: u32) {

}

fn delete_address(curr_list: &LinkedList<AddressBook>, id: u32) {

}

fn save_address_list(curr_list: &LinkedList<AddressBook>) {
    /*
    let addr_book_file = File::create("addr_book.ab").unwrap();
    let mut buf_writer_addr = BufWriter::new(addr_book_file);

    for addr_book in curr_list {
        buf_writer_addr.write(&addr_book.name.into_bytes());
    }

    buf_writer_addr.flush();
    */
}
