use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If table is empty (no headers), print nothing
        if self.headers.is_empty() {
            return Ok(());
        }

        // Calculate column widths
        let mut col_widths = Vec::new();
        
        // Initialize with header lengths
        for header in &self.headers {
            col_widths.push(header.len());
        }
        
        // Check body rows for maximum width in each column
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        // Print header row
        write!(f, "|")?;
        for (i, header) in self.headers.iter().enumerate() {
            let width = col_widths[i];
            let centered = center_text(header, width);
            write!(f, " {} |", centered)?;
        }
        writeln!(f)?;

        // Print separator row
        write!(f, "|")?;
        for (i, _) in self.headers.iter().enumerate() {
            let width = col_widths[i];
            write!(f, "{}", "-".repeat(width + 2))?;
            if i < self.headers.len() - 1 {
                write!(f, "+")?;
            }
        }
        writeln!(f, "|")?;

        // Print body rows
        for row in &self.body {
            write!(f, "|")?;
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let width = col_widths[i];
                    let centered = center_text(cell, width);
                    write!(f, " {} |", centered)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

// Helper function to center text within a given width
fn center_text(text: &str, width: usize) -> String {
    if text.len() >= width {
        return text.to_string();
    }
    
    let total_padding = width - text.len();
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    
    format!("{}{}{}", 
            " ".repeat(left_padding), 
            text, 
            " ".repeat(right_padding))
}

fn main() {
    let mut table = Table::new();
    println!("{}", table);
    table.headers = vec![
        String::from("Model"),
        String::from("Piece N°"),
        String::from("In Stock"),
        String::from("Description"),
    ];
    table.add_row(&[
        String::from("model 1"),
        String::from("43-EWQE304"),
        String::from("30"),
        String::from("Piece for x"),
    ]);
    table.add_row(&[
        String::from("model 2"),
        String::from("98-QCVX5433"),
        String::from("100000000"),
        String::from("-"),
    ]);
    table.add_row(&[
        String::from("model y"),
        String::from("78-NMNH"),
        String::from("60"),
        String::from("nothing"),
    ]);
    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_table() {
        let table = Table::new();
        assert_eq!(format!("{}", table), "");
    }

    #[test]
    fn test_center_text() {
        assert_eq!(center_text("hi", 6), "  hi  ");
        assert_eq!(center_text("hello", 7), " hello ");
        assert_eq!(center_text("test", 5), " test"); // Slightly to the left when can't center exactly
        assert_eq!(center_text("same", 4), "same");
    }

    #[test]
    fn test_table_with_headers_only() {
        let mut table = Table::new();
        table.headers = vec![String::from("A"), String::from("B")];
        let output = format!("{}", table);
        assert!(output.contains("| A | B |"));
        assert!(output.contains("|---|---|"));
    }

    #[test]
    fn test_table_with_data() {
        let mut table = Table::new();
        table.headers = vec![String::from("Name"), String::from("Age")];
        table.add_row(&[String::from("Alice"), String::from("25")]);
        table.add_row(&[String::from("Bob"), String::from("30")]);
        
        let output = format!("{}", table);
        assert!(output.contains("| Name | Age |"));
        assert!(output.contains("| Alice | 25 |"));
        assert!(output.contains("| Bob  | 30 |"));
    }

    #[test]
    fn test_column_width_adjustment() {
        let mut table = Table::new();
        table.headers = vec![String::from("Short"), String::from("Long")];
        table.add_row(&[String::from("VeryLongContent"), String::from("X")]);
        
        let output = format!("{}", table);
        // The first column should expand to fit "VeryLongContent"
        assert!(output.contains("VeryLongContent"));
        assert!(output.contains("Short"));
    }

    #[test]
    fn test_centering_behavior() {
        let mut table = Table::new();
        table.headers = vec![String::from("Test")];
        table.add_row(&[String::from("LongContent")]);
        
        let output = format!("{}", table);
        // "Test" should be centered in the wider column
        assert!(output.contains(" Test "));
    }
}
