
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

enum MenuState {
    Normal,
    Deleting,
    Modifying,
    Inserting
}

impl PartialEq for MenuState {
    fn eq(&self, other: &MenuState) -> bool {
        match (self, other) {
            (&MenuState::Normal, &MenuState::Normal) => true,
            (&MenuState::Inserting, &MenuState::Inserting) => true,
            (&MenuState::Modifying, &MenuState::Modifying) => true,
            (&MenuState::Deleting, &MenuState::Deleting) => true,
            _ => false,
        }
    }
}

// Struct to hold each Address in the Address Book
// This Name actually kinda sucks, but it's fine for me
#[derive(Clone)]    // This is used to inherit the Trait Clone in the Struct AddressBook
struct AddressBook {
    id: u32,
    name: String,
    address: String,
    postcode: String,
    city: String,
    country: String
}

/* Implement various Functions for the Struct AddressBook
 */
impl AddressBook {
    // Constructor
    fn new(n_id: u32, n_name: String, n_address: String, n_postcode: String, n_city: String, n_country: String) -> AddressBook {
        // No Return needed, if no Return is stated, the last line is returned
        AddressBook {
            id: n_id,
            name: n_name,
            address: n_address,
            postcode: n_postcode,
            city: n_city,
            country: n_country
        }
    }

    // Reset all values in this Struct.
    fn reset(&mut self) {
        self.id = 0;
        self.name = "".to_string();
        self.address = "".to_string();
        self.postcode = "".to_string();
        self.city = "".to_string();
        self.country = "".to_string();
    }

    // Formatted Printing of the Address
    fn print_address(&self) {
        println!("AddressBook {{");
        println!("  id      : {}", self.id);
        println!("  name    : {}", self.name);
        println!("  address : {}", self.address);
        println!("  postcode: {}", self.postcode);
        println!("  city    : {}", self.city);
        println!("  country : {}", self.country);
        println!("}}");
    }
}

fn main() {
    let reader = io::stdin();
    let mut command;
    let mut m_state: MenuState = MenuState::Normal;
    let mut id_selection = false;

    // load the list of existing addresses into the List of AddresBook.
    let mut addr_list: LinkedList<AddressBook> = load_existing_addressbook();

    println!("Welcome to the Address Book Manager. You have the following options at your disposal:");
    println!("1. Create/Add address");
    println!("2. Modify existing address");
    println!("3. Delete existing address");
    println!("4. Show current addresses");
    println!("5. Reload from file (Default Filename: {})", DEFAULT_FILE_NAME);
    println!("6. Exit/Quit");

    for line in reader.lock().lines()   {
        command = line.unwrap();

        if m_state == MenuState::Normal {
            match &*command.to_lowercase() {
                "1"|"add"|"create"  => {
                    id_selection = false;
                    m_state = MenuState::Inserting;
                    println!("Please type the new address in the next line. The ID will be set automatically.");
                    println!("Use this format: ([Name];[Street];[Postcode];[City];[Country])");
                }
                "2"|"modify"        => {
                    ;
                }
                "3"|"delete"        => {
                    id_selection = true;
                    m_state = MenuState::Deleting;
                    println!("Which ID do you want to delete?");
                }
                "4"|"show"|"current"=> {
                    print_address_list(&addr_list);
                    println!("What do you want to do now?");
                }
                "5"|"reload"        => {
                    addr_list = load_existing_addressbook();
                    println!("Successfully loaded addresses from file.");
                }
                "6"|"quit"|"exit"   => {
                    save_address_list(&addr_list);
                    break;
                }
                _               => {
                    println!("The command '{}' was not found.", &*command.to_lowercase());
                }
            }
        } else {
            match m_state {
                MenuState::Inserting => {
                    addr_list = create_new_address(command.clone(), &addr_list);
                    println!("Successfully added the new Address.");
                    m_state = MenuState::Normal;
                    id_selection = false;
                },
                MenuState::Modifying => {

                },
                MenuState::Deleting => {
                    if id_selection {
                        match command.parse::<u32>() {
                            Ok(n) => {
                                addr_list = delete_address(&addr_list, n);
                                println!("Address {} was deleted.", n);
                                m_state = MenuState::Normal;
                                id_selection = false;
                            },
                            Err(e) => {
                                m_state = MenuState::Normal;
                            },
                        }
                    }
                }
                _ => {
                    // Do nothing
                }
            };
        }
        command.clear();
    }
}

// Loads the addressbook from the current directory into a Linked List.
fn load_existing_addressbook() -> LinkedList<AddressBook> {
    let mut addr_book_list: LinkedList<AddressBook> = LinkedList::new();

    let fname = DEFAULT_FILE_NAME;
    let file = match File::open(fname) {
        Ok(file) => file,
        Err(e) => panic!("File error: {}", e)
    };
    let reader = BufReader::new(file);

    let mut is_new_instance = false;
    let mut curr_addr_book = AddressBook::new(0,format!(""),format!(""),format!(""),
                                                format!(""),format!(""));

    for line in reader.lines() {
        let mut linebuffer = line.unwrap_or("".to_string());
        if linebuffer.len() >= INSTANCE_SEPERATOR.len() {
            if &linebuffer[..INSTANCE_SEPERATOR.len()] == INSTANCE_SEPERATOR {
                if is_new_instance {
                    is_new_instance = false;
                    addr_book_list.push_back(curr_addr_book.clone());
                } else {
                    is_new_instance = true;
                    curr_addr_book.reset();
                }
            }
        }
        if linebuffer.len() >= 10 {
            match &linebuffer[..10] {
                "id      : "    => {
                    match (&linebuffer[10..]).parse::<u32>() {
                        Ok(n) => curr_addr_book.id = n,
                        Err(e) => curr_addr_book.id = 0,
                    }
                },
                "name    : "    => {
                    curr_addr_book.name = (&linebuffer[10..]).to_string();
                },
                "address : "    => {
                    curr_addr_book.address = (&linebuffer[10..]).to_string();
                },
                "postcode: "    => {
                    curr_addr_book.postcode = (&linebuffer[10..]).to_string();
                },
                "city    : "    => {
                    curr_addr_book.city = (&linebuffer[10..]).to_string();
                },
                "country : "    => {
                    curr_addr_book.country = (&linebuffer[10..]).to_string();
                },
                _               => {
                    println!("couln't match: '{}'", linebuffer);
                }
            }
        }
    }

    return addr_book_list;
}

fn create_new_address(creation_string: String, curr_list: &LinkedList<AddressBook>) -> LinkedList<AddressBook> {
    let mut new_list: LinkedList<AddressBook> = curr_list.clone();
    let values = split_string_into_addr_array(&creation_string);

    let new_id = get_next_id(&new_list);
    let new_addr = AddressBook::new(new_id,
                                    values[0].clone(),
                                    values[1].clone(),
                                    values[2].clone(),
                                    values[3].clone(),
                                    values[4].clone());
    new_list.push_back(new_addr);

    return new_list;
}

fn modify_address(curr_list: &mut LinkedList<AddressBook>, id: u32) {

}

/* A not very memory-friendly way of deleting an element from a LinkedList without having to directly
 * modify the List.
 */
fn delete_address(curr_list: &LinkedList<AddressBook>, id: u32) -> LinkedList<AddressBook> {
    let mut new_list: LinkedList<AddressBook> = LinkedList::new();

    for addr_book in curr_list {
        if addr_book.id != id {
            new_list.push_back(addr_book.clone());
        }
    }

    return new_list;
}

/* Prints each Address from the AddressBook List.
 */
fn print_address_list(curr_list: &LinkedList<AddressBook>) {
    for addr_book in curr_list {
        addr_book.print_address();
    }
}

/* The Function save_address_list(curr_list: &LinkedList<AddressBook>) takes a Linked List of AddressBook
 * addresses and just writes each of them to the default File that is defined by the constant in the top
 * of the code. This function panics if there are any difficulties with the file creation/modification.
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

        linebuffer = "id      : ".to_string() + &(addr_book.id.to_string()) + "\n";
        writer.write(&linebuffer.into_bytes());

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

/* Iterates over the AddressBook List to check which id is the next available
 * id that can be used
 */
fn get_next_id(curr_list: &LinkedList<AddressBook>) -> u32 {
    let mut max_id = 0;
    for addr_book in curr_list {
        if max_id < addr_book.id {
            max_id = addr_book.id;
        }
    }
    return max_id + 1;
}

/* Splits an Input String into the respective fields of an Array.
 * This array is used to create a new AddressBook instance from it.
 */
fn split_string_into_addr_array(input_string: &str) -> Vec<String> {
    let mut addr_arr = vec!["".to_string(); 5];
    let mut index = 0;
    let mut counter = 0;

    for str_char in input_string.chars() {
        if ((counter != 0) && (counter != input_string.len()-1)) ||
            ((str_char != '(') && (str_char != ')'))
        {
            if str_char == ';' {
                index += 1;
            } else {
                addr_arr[index].push(str_char);
            }
        }

        counter += 1;
    }

    return addr_arr;
}
