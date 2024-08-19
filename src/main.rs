// Problem 1:
/* You are tasked with implementing a library management system using Rust.
Your goal is to design a program that can handle books and magazines.
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input
and displays its information. The function should output
the item's ID, title, publication year, and type (book or magazine).
*/

enum ItemType {
    Book,
    Magazine,
}

impl ItemType {
    fn get_type(&self) -> String {
        let a = match self {
            ItemType::Book => "Book".to_string(),
            ItemType::Magazine => "Magazine".to_string(),
        };
        a
    }
}

struct Item {
    id: u32,
    title: String,
    year: u32,
    item_type: ItemType,
}

impl Item {
    fn display_item_info(&self) {
        let t: String = self.item_type.get_type();
        println!(
            "Id : {}, title : {}, year: {}, type : {}",
            self.id, self.title, self.year, t
        );
    }
}

fn main() {
    let book1 = Item {
        id: 1,
        title: "Adventure of Sherlock Holmes".to_string(),
        year: 1892,
        item_type: ItemType::Book,
    };
    book1.display_item_info();

    let book2 = Item {
        id: 2,
        title: "Top Gear".to_string(),
        year: 2021,
        item_type: ItemType::Magazine,
    };
    book2.display_item_info();
}
