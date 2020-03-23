
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;

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
    ModifySelection,
    Modifying,
    Inserting
}

impl PartialEq for MenuState {
    fn eq(&self, other: &MenuState) -> bool {
        match (self, other) {
            (&MenuState::Normal, &MenuState::Normal) => true,
            (&MenuState::Inserting, &MenuState::Inserting) => true,
            (&MenuState::ModifySelection, &MenuState::ModifySelection) => true,
            (&MenuState::Modifying, &MenuState::Modifying) => true,
            (&MenuState::Deleting, &MenuState::Deleting) => true,
            _ => false,
        }
    }
}

// Struct to hold each Address in the Address Book
#[derive(Clone)]    // This is used to inherit the Trait Clone in the Struct Address
struct Address {
    name: String,
    address: String,
    postcode: String,
    city: String,
    country: String
}

/* Implement various Functions for the Struct Address
 */
impl Address {
    // Constructor
    fn new(n_name: &str, n_address: &str, n_postcode: &str, n_city: &str, n_country: &str) -> Address {
        // No Return needed, if no Return is stated, the last line is returned
        Address {
            name: String::from(n_name),
            address: String::from(n_address),
            postcode: String::from(n_postcode),
            city: String::from(n_city),
            country: String::from(n_country)
        }
    }

    // Reset all values in this Struct.
    fn reset(&mut self) {
        self.name = String::from("");
        self.address = String::from("");
        self.postcode = String::from("");
        self.city = String::from("");
        self.country = String::from("");
    }

    // Formatted Printing of the Address
    fn print_address(&self) {
        println!("  name    : {}", self.name);
        println!("  address : {}", self.address);
        println!("  postcode: {}", self.postcode);
        println!("  city    : {}", self.city);
        println!("  country : {}", self.country);
    }
}

fn main() {
    let reader = io::stdin();
    let mut command;
    let mut m_state: MenuState = MenuState::Normal;
    let mut id_selection = false;
    let mut curr_id: usize = 0;

    // load the list of existing addresses into the Vector of Addresses.
    let mut addr_vec: Vec<Address> = load_existing_addressbook();

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
            curr_id = 0;
            match &*command.to_lowercase() {
                "1"|"add"|"create"  => {
                    id_selection = false;
                    m_state = MenuState::Inserting;
                    println!("Please type the new address in the next line. The ID will be set automatically.");
                    println!("Use this format: ([Name];[Street];[Postcode];[City];[Country])");
                }
                "2"|"modify"        => {
                    id_selection = true;
                    m_state = MenuState::ModifySelection;
                    println!("Which ID do you want to modify?");
                }
                "3"|"delete"        => {
                    id_selection = true;
                    m_state = MenuState::Deleting;
                    println!("Which ID do you want to delete?");
                }
                "4"|"show"|"current"=> {
                    print_address_list(&addr_vec);
                    println!("What do you want to do now?");
                }
                "5"|"reload"        => {
                    addr_vec = load_existing_addressbook();
                    println!("Successfully loaded addresses from file.");
                }
                "6"|"quit"|"exit"   => {
                    save_address_list(&addr_vec);
                    break;
                }
                _               => {
                    println!("The command '{}' was not found.", &*command.to_lowercase());
                }
            }
        } else {
            if id_selection {
                match command.parse::<usize>() {
                    Ok(n) => {
                        curr_id = n;
                        id_selection = false;
                    },
                    Err(e) => {
                        curr_id = 0;
                        m_state = MenuState::Normal;
                        id_selection = false;
                    },
                }
            }
            match m_state {
                MenuState::Inserting => {
                    add_new_address(command.clone(), &mut addr_vec);
                    println!("Successfully added the new Address.");
                    m_state = MenuState::Normal;
                    id_selection = false;
                },
                MenuState::ModifySelection => {
                    println!("This is the current Adress:");
                    print_single_address_from_list(&addr_vec, curr_id-1);
                    println!("Please type the new address in the next line. The ID will be set automatically.");
                    println!("Use this format: ([Name];[Street];[Postcode];[City];[Country])");
                    m_state = MenuState::Modifying;
                    id_selection = false;
                }
                MenuState::Modifying => {
                    modify_address(command.clone(), &mut addr_vec, curr_id-1);
                    println!("Address {} was modified.", curr_id);
                    m_state = MenuState::Normal;
                    id_selection = false;
                },
                MenuState::Deleting => {
                    delete_address(&mut addr_vec, curr_id);
                    println!("Address {} was deleted.", curr_id-1);
                    m_state = MenuState::Normal;
                    id_selection = false;
                }
                _ => {
                    // Do nothing
                }
            };
        }
        command.clear();
    }
}

// Loads the addressbook from the current directory into a Vector.
fn load_existing_addressbook() -> Vec<Address> {
    let mut addr_vec: Vec<Address> = Vec::new();

    let fname = DEFAULT_FILE_NAME;
    let file = match File::open(fname) {
        Ok(file) => file,
        Err(e) => panic!("File error: {}", e)
    };
    let reader = BufReader::new(file);

    let mut is_new_instance = false;
    let mut curr_addr = Address::new("","","","","");

    for line in reader.lines() {
        let mut linebuffer = line.unwrap_or("".to_string());
        if linebuffer.len() >= INSTANCE_SEPERATOR.len() {
            if &linebuffer[..INSTANCE_SEPERATOR.len()] == INSTANCE_SEPERATOR {
                if is_new_instance {
                    is_new_instance = false;
                    addr_vec.push(curr_addr.clone());
                } else {
                    is_new_instance = true;
                    curr_addr.reset();
                }
            }
        }
        if linebuffer.len() >= 10 {
            match &linebuffer[..10] {
                "name    : "    => {
                    curr_addr.name = (&linebuffer[10..]).to_string();
                },
                "address : "    => {
                    curr_addr.address = (&linebuffer[10..]).to_string();
                },
                "postcode: "    => {
                    curr_addr.postcode = (&linebuffer[10..]).to_string();
                },
                "city    : "    => {
                    curr_addr.city = (&linebuffer[10..]).to_string();
                },
                "country : "    => {
                    curr_addr.country = (&linebuffer[10..]).to_string();
                },
                _               => {
                    println!("couln't match: '{}'", linebuffer);
                }
            }
        }
    }

    return addr_vec;
}

fn add_new_address(creation_string: String, curr_vec: &mut Vec<Address>) {
    let values = split_string_into_addr_array(&creation_string);
    let new_addr = Address::new(&values[0],
                                &values[1],
                                &values[2],
                                &values[3],
                                &values[4]);
    curr_vec.push(new_addr);
}

/* Simply Modify an Address by deleting the old one and creating a new one.
 */
fn modify_address(creation_string: String, mut curr_vec: &mut Vec<Address>, id: usize) {
    delete_address(&mut curr_vec, id);
    let values = split_string_into_addr_array(&creation_string);
    let new_addr = Address::new(&values[0],
                                &values[1],
                                &values[2],
                                &values[3],
                                &values[4]);
    curr_vec.insert(id, new_addr);
}

/* Delete the selected id from the Vector
 */
fn delete_address(curr_vec: &mut Vec<Address>, id: usize) {
    curr_vec.remove(id);
}

/* Prints each Address from the Address Vector.
 */
fn print_address_list(curr_vec: &Vec<Address>) {
    let mut i: u32 = 0;
    for addr in curr_vec {
        println!("Address [{}]", i+1);
        addr.print_address();
        i += 1;
    }
}

/* Prints a single address with the selected ID.
 */
fn print_single_address_from_list(curr_vec: &Vec<Address>, id: usize) {
    match curr_vec.get(id) {
        Some(addr) => addr.print_address(),
        None => ()
    }
}

/* The Function save_address_list(curr_vec: &Vec<Address>) takes a Vector of Address
 * addresses and just writes each of them to the default File that is defined by the constant in the top
 * of the code. This function panics if there are any difficulties with the file creation/modification.
 */
fn save_address_list(curr_vec: &Vec<Address>) {
    let fname = DEFAULT_FILE_NAME;

    let mut file = match File::create(fname) {
        Ok(file) => file,
        Err(e) => panic!("File error: {}", e)
    };

    let mut writer = BufWriter::new(&file);
    for addr in curr_vec {
        let mut linebuffer = INSTANCE_SEPERATOR.to_string() + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "name    : ".to_string() + &addr.name + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "address : ".to_string() + &addr.address + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "postcode: ".to_string() + &addr.postcode + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "city    : ".to_string() + &addr.city + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = "country : ".to_string() + &addr.country + "\n";
        writer.write(&linebuffer.into_bytes());

        linebuffer = INSTANCE_SEPERATOR.to_string() + "\n";
        writer.write(&linebuffer.into_bytes());
    }

    writer.flush();
}

/* Splits an Input String into the respective fields of an Array.
 * This array is used to create a new Address instance from it.
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
