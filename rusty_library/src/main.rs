use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Local, DateTime};

use anyhow::{bail, Result};

fn main() {

    let harry_potter: Book = Book {
        title: String::from("Harry Potter"),
        writer: String::from("J K Rowling"),
        published_date: NaiveDate::parse_from_str("1997-07-26", "%Y-%m-%d").unwrap()
    };

    let eragon: Book = Book {
        title: String::from("Eragon"),
        writer: String::from("Christopher Paolini"),
        published_date: NaiveDate::parse_from_str("2003-08-26", "%Y-%m-%d").unwrap()
    };

    let books = vec![harry_potter, eragon];

    let mut library_a: Library = Library {
        name: String::from("Library A"),
        books: HashMap::new(),
        stock: Stock {
             available: HashMap::new(),
             borrowed: HashMap::new()
        }
    };

    for book in books {
        library_a.books.insert(book.title.clone(), book.clone());
        let title = book.title.clone();
        if title == "Harry Potter" {
            library_a.stock.available.insert(title.clone(), 10);
            library_a.stock.borrowed.insert(title.clone(), 3);
        } else if title == "Eragon" {
            library_a.stock.available.insert(title.clone(), 10);
            library_a.stock.borrowed.insert(title.clone(), 4);
        }
    };

    let _ = library_a.add_new_stock("Harry Potter", 100);
    let _ = library_a.return_book("Eragon", 3);
    let _ = library_a.borrow_a_book("Eragon", 100);

    println!("{:?}", library_a)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// kita mendefinision buku dan semua atribut informasi tentang buku
struct Book {
    title: String,
    writer: String,
    published_date: NaiveDate
}
#[derive(Debug)]
struct Stock {
    available: HashMap<String, u64>,
    borrowed: HashMap<String, u64>
}

struct Member {
    name: String,
    library: Library // library identity where the member was registered to
}

// library is the same as warehouse name
// it stores the book inside theirs
#[derive(Debug)]
struct Library {
    name: String,
    books: HashMap<String, Book>,
    stock: Stock,
}

// we need to implement some methods into our Stock to do some
// automatic calculation.
// Every amount we add to the Stock method must be an positive integer number
// We can only add registered books into the stock system
impl Library {
    fn check_book(&mut self, title: &str) -> Result<()> {
        if !self.books.contains_key(title) {
            bail!("No book with the title of {title}")
        }

        Ok(())
    }
    // this function is only to add the stock
    // not caring about how much stock is available
    fn add_new_stock(&mut self, title: &str, amount: u64) -> Result<()> {
        self.check_book(title)?;
        let available_stock = self.stock.available
            .entry(title.to_string())
            .or_insert(0);

        let current = *available_stock;
        *available_stock += amount;
        println!("Stock has beed added from {current} to {available_stock}");

        Ok(())
    }

    // this function is only to remove the stock
    // not caring about how much stock is available
    fn remove_stock(&mut self, title: &str, amount: u64) -> Result<()> {
        self.check_book(title)?;
        if let Some(available_stock) = self.stock.available.get_mut(title) {
            if *available_stock == 0 || *available_stock < amount {
                bail!("Can not reduce stok. Current stok: {available_stock}, amount to reduce: {amount}");
            }
            let current = *available_stock;
            *available_stock -= amount;
            println!("Stock has beed reduced from {current} to {available_stock}");
        }

        Ok(())
    }

    fn add_borrowing(&mut self, title: &str, amount: u64) -> Result<()> {
        self.check_book(title)?;
        let borowed_stock = self.stock.borrowed
            .entry(title.to_string())
            .or_insert(0);
        *borowed_stock += amount;
        println!("{amount} Book of {title} has beed borrowed!");

        Ok(())
    }

    fn reduce_borrowing(&mut self, title: &str, amount: u64) -> Result<()> {
        self.check_book(title)?;
        if let Some(borrowed_qty) = self.stock.borrowed.get_mut(title) {
            if *borrowed_qty < amount {
                bail!("Returned amount: {amount} is bigger than borrowed: {borrowed_qty}");
            }
            *borrowed_qty -= amount;
            println!("{amount} Book of {title} has beed returned!");
        }
        Ok(())
    }

    // reduce when we want to reduce a stock due to borrowing
    fn borrow_a_book(&mut self, title: &str, amount: u64) -> Result<()> {
        self.remove_stock(title, amount)?;
        self.add_borrowing(title, amount)?;

        Ok(())
    }

    fn return_book(&mut self, title: &str, amount: u64) -> Result<()> {
        self.reduce_borrowing(title, amount)?;
        self.add_new_stock(title, amount)?;

        Ok(())
    }
}
