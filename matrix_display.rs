pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        // Convert the slice of slices into a Vec<Vec<i32>>
        let data = slice.iter().map(|row| row.to_vec()).collect();
        Matrix(data)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Iterate through each row
        for (row_index, row) in self.0.iter().enumerate() {
            // Add newline before each row except the first
            if row_index > 0 {
                writeln!(f)?;
            }
            // Write opening parenthesis
            write!(f, "(")?;
            // Write elements with spaces between them
            for (i, &num) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", num)?;
            }
            // Write closing parenthesis
            write!(f, ")")?;
        }
        Ok(())
    }
}