pub mod writers {
    use super::books::Book;

    pub struct Writer {
        pub first_name: String,
        pub last_name: String,
        pub books: Vec<Book>,
    }
}

pub mod books {
    pub struct Book {
        pub title: String,
        pub year: u64,
    }
}

pub fn order_books(writer: &mut writers::Writer) {
    writer.books.sort_by(|a, b| a.title.cmp(&b.title));
}

// Example usage and tests
pub use writers::Writer;
pub use books::Book;

fn main() {
    let mut writer_a = Writer {
        first_name: "William".to_string(),
        last_name: "Shakespeare".to_string(),
        books: vec![
            Book {
                title: "Hamlet".to_string(),
                year: 1600,
            },
            Book {
                title: "Othelo".to_string(),
                year: 1603,
            },
            Book {
                title: "Romeo and Juliet".to_string(),
                year: 1593,
            },
            Book {
                title: "MacBeth".to_string(),
                year: 1605,
            },
        ],
    };

    println!("Before ordering");
    for b in &writer_a.books {
        println!("{:?}", b.title);
    }

    order_books(&mut writer_a);

    println!("\nAfter ordering");
    for b in writer_a.books {
        println!("{:?}", b.title);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_creation() {
        let book = Book {
            title: "Test Book".to_string(),
            year: 2023,
        };
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.year, 2023);
    }

    #[test]
    fn test_writer_creation() {
        let writer = Writer {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            books: vec![],
        };
        assert_eq!(writer.first_name, "Jane");
        assert_eq!(writer.last_name, "Doe");
        assert_eq!(writer.books.len(), 0);
    }

    #[test]
    fn test_order_books() {
        let mut writer = Writer {
            first_name: "Test".to_string(),
            last_name: "Author".to_string(),
            books: vec![
                Book {
                    title: "Zebra".to_string(),
                    year: 2000,
                },
                Book {
                    title: "Apple".to_string(),
                    year: 2001,
                },
                Book {
                    title: "Banana".to_string(),
                    year: 2002,
                },
            ],
        };

        order_books(&mut writer);

        assert_eq!(writer.books[0].title, "Apple");
        assert_eq!(writer.books[1].title, "Banana");
        assert_eq!(writer.books[2].title, "Zebra");
    }

    #[test]
    fn test_order_books_shakespeare_example() {
        let mut writer = Writer {
            first_name: "William".to_string(),
            last_name: "Shakespeare".to_string(),
            books: vec![
                Book {
                    title: "Hamlet".to_string(),
                    year: 1600,
                },
                Book {
                    title: "Othelo".to_string(),
                    year: 1603,
                },
                Book {
                    title: "Romeo and Juliet".to_string(),
                    year: 1593,
                },
                Book {
                    title: "MacBeth".to_string(),
                    year: 1605,
                },
            ],
        };

        order_books(&mut writer);

        let expected_order = vec!["Hamlet", "MacBeth", "Othelo", "Romeo and Juliet"];
        for (i, book) in writer.books.iter().enumerate() {
            assert_eq!(book.title, expected_order[i]);
        }
    }

    #[test]
    fn test_empty_books_list() {
        let mut writer = Writer {
            first_name: "Empty".to_string(),
            last_name: "Writer".to_string(),
            books: vec![],
        };

        order_books(&mut writer);
        assert_eq!(writer.books.len(), 0);
    }

    #[test]
    fn test_single_book() {
        let mut writer = Writer {
            first_name: "Single".to_string(),
            last_name: "Book".to_string(),
            books: vec![
                Book {
                    title: "Only Book".to_string(),
                    year: 2023,
                },
            ],
        };

        order_books(&mut writer);
        assert_eq!(writer.books.len(), 1);
        assert_eq!(writer.books[0].title, "Only Book");
    }
}
