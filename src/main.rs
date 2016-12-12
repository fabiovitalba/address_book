
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::collections::LinkedList;

// Difference between String and &str:
// String is like a Vector of expandable buffer full of characters on the heap
// &str is pointer to an immutable string somewhere
// Stolen from kazagistar from the official Rust IRC

// Using some Constants for ease of use.
// These could be outsourced to a config file somewhat later maybe.
static INSTANCE_SEPERATOR: &'static str = "***";
static DEFAULT_FILE_NAME: &'static str = "addr_book.ab";

// Struct to hold each Address in the Address Book
// This Name actually kinda sucks, but it's fine for me
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
    let fname = DEFAULT_FILE_NAME;
    let file = match File::open(fname) {
        Ok(file) => file,
        Err(e) => panic!("File error: {}", e)
    };
    let reader = BufReader::new(file);

    let mut is_new_instance = false;
    let mut curr_addr_book;
    //let mut id, name, address, postcode, city, country;

    for line in reader.lines() {
        let mut linebuffer = line.unwrap_or("".to_string());
        if curr_line.len() > 3 {
            if curr_line[..3] == INSTANCE_SEPERATOR.to_string() {
                if is_new_instance {
                    is_new_instance = false;

                } else {
                    is_new_instance = true;
                }
            }
            // TODO: match each value from the file to a respective value from the struct.
            /*
            match {

                "name    : "    => {},
                _               => {}
            }
            */
        }
    }

    return LinkedList::new();
}

fn create_new_address(curr_list: &LinkedList<AddressBook>) {

}

fn modify_address(curr_list: &LinkedList<AddressBook>, id: u32) {

}

fn delete_address(curr_list: &LinkedList<AddressBook>, id: u32) {

}

/* The Function save_address_list(curr_list: &LinkedList<AddressBook>) takes a Linked List of AddressBook addresses
 * and just writes each of them to the default File that is defined by the constant in the top of the code.
 * This function panics if there are any difficulties with the file creation/modification.
 */
fn save_address_list(curr_list: &LinkedList<AddressBook>) {
    let fname = DEFAULT_FILE_NAME;

    let mut file = match File::create(fname) {
        Ok(file) => file,
        Err(e) => panic!("File error: {}", e)
    };

    let mut writer = BufWriter::new(&file);
    for addr_book in curr_list {
        let mut linebuffer = INSTANCE_SEPERATOR.to_string() + "\n";
        writer.write(&linebuffer.into_bytes());

        //TODO: convert the id to string and then into bytes. Or directly into bytes?
        //linebuffer = "id      : ".to_string() + &(addr_book.id.into_string()) + "\n";
        //writer.write(&linebuffer.into_bytes());

        linebuffer = "name    : ".to_string() + &addr_book.name + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "address : ".to_string() + &addr_book.address + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "postcode: ".to_string() + &addr_book.postcode + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "city    : ".to_string() + &addr_book.city + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "country : ".to_string() + &addr_book.country + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = INSTANCE_SEPERATOR.to_string() + "\n";
        writer.write(&linebuffer.into_bytes());
    }

    writer.flush();
}
