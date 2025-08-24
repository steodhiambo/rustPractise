use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table { headers: vec![], body: vec![] }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    pub fn filter_col<F: Fn(&str) -> bool>(&self, filter: F) -> Option<Self> {
        let idx: Vec<usize> = self.headers.iter()
            .enumerate()
            .filter_map(|(i, h)| filter(h).then_some(i))
            .collect();
        
        if idx.is_empty() {
            return None;
        }
        
        Some(Table {
            headers: idx.iter().map(|&i| self.headers[i].clone()).collect(),
            body: self.body.iter()
                .map(|r| idx.iter().map(|&i| r[i].clone()).collect())
                .collect(),
        })
    }

    pub fn filter_row<F: Fn(&str) -> bool>(&self, col_name: &str, filter: F) -> Option<Self> {
        let col = self.headers.iter().position(|h| h == col_name)?;
        let rows: Vec<_> = self.body.iter()
            .filter(|r| filter(&r[col]))
            .cloned()
            .collect();
        
        if rows.is_empty() {
            return None;
        }
        
        Some(Table { 
            headers: self.headers.clone(), 
            body: rows 
        })
    }
}

// Implement Display trait for Table to enable pretty printing
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.headers.is_empty() {
            return write!(f, "Empty table");
        }

        // Calculate column widths
        let mut col_widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();
        
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        // Print headers
        write!(f, "|")?;
        for (i, header) in self.headers.iter().enumerate() {
            write!(f, " {:^width$} |", header, width = col_widths[i])?;
        }
        writeln!(f)?;

        // Print separator line
        write!(f, "|")?;
        for &width in &col_widths {
            write!(f, "-{:-<width$}-|", "", width = width)?;
        }
        writeln!(f)?;

        // Print rows
        for row in &self.body {
            write!(f, "|")?;
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    write!(f, " {:^width$} |", cell, width = col_widths[i])?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}