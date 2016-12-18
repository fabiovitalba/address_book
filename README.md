# Address Book Manager
This is a small program I wrote in Rust that handles the addresses written in a .ab-file and lets you modify them.
When closing the program, it automatically saves all the addresses to the file.

## Inserting/Creating addresses
To insert/create a new Address you call one of the following commands: *1*, *add*, *create* and then type the new address in one line in the following format: *(Name;Street;Postcode;City;Country)*. The Brackets are optional and you may leave some values blank.
The ID will be calculated automatically based on the other Addresses. The highest ID will be found and increased by 1. That will be the new ID.

## Modifying Addresses
Work in Progress

## Deleting Addresses
To delete an existing address you call one of these commands: *3*,*delete* and then type the ID of the Address you want to delete.

## Loading from file
You can call the function to reload from the Address Book file (.ab) with one of these commands: *5*, *reload*.
In that case the program will be looking for a file called "addr_book.ab". When found it will load the addresses found in the file.

## Saving to file
The Address Book List is saved as soon as you exit the program. You can exit the program traditionally by using one of the following commands: *6*, *quit*, *exit*.

## File format for addr_book.ab
The File format for this Address list looks like so:
\*\*\*
id      : *ID*
name    : *NAME*
address : *STREET AND NO.*
postcode: *POST CODE*
city    : *CITY*
country : *COUNTRY*
\*\*\*
