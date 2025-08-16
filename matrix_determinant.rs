pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    // Extract elements from the first row for cofactor expansion
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];
    
    // Calculate 2x2 determinants for each cofactor
    // For element 'a' at (0,0): remove row 0 and column 0
    let det_a = matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1];
    
    // For element 'b' at (0,1): remove row 0 and column 1
    let det_b = matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0];
    
    // For element 'c' at (0,2): remove row 0 and column 2
    let det_c = matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0];
    
    // Apply cofactor expansion: a*det_a - b*det_b + c*det_c
    a * det_a - b * det_b + c * det_c
}

fn main() {
    let matrix = [[1, 2, 4], [2, -1, 3], [4, 0, 1]];

    println!(
        "The determinant of the matrix:\n|1  2  4|\n|2 -1  3|  = {}\n|4  0  1|",
        matrix_determinant(matrix)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_matrix() {
        let matrix = [[1, 2, 4], [2, -1, 3], [4, 0, 1]];
        assert_eq!(matrix_determinant(matrix), 35);
    }

    #[test]
    fn test_identity_matrix() {
        let matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        assert_eq!(matrix_determinant(matrix), 1);
    }

    #[test]
    fn test_zero_determinant() {
        // Matrix with two identical rows should have determinant 0
        let matrix = [[1, 2, 3], [1, 2, 3], [4, 5, 6]];
        assert_eq!(matrix_determinant(matrix), 0);
    }

    #[test]
    fn test_negative_determinant() {
        let matrix = [[2, 1, 3], [1, 0, 1], [1, 2, 1]];
        assert_eq!(matrix_determinant(matrix), -2);
    }

    #[test]
    fn test_zero_matrix() {
        let matrix = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        assert_eq!(matrix_determinant(matrix), 0);
    }

    #[test]
    fn test_upper_triangular_matrix() {
        // Upper triangular matrix determinant = product of diagonal elements
        let matrix = [[2, 3, 1], [0, 4, 5], [0, 0, 6]];
        assert_eq!(matrix_determinant(matrix), 2 * 4 * 6); // = 48
    }

    #[test]
    fn test_negative_values() {
        let matrix = [[-1, 2, -3], [4, -5, 6], [-7, 8, -9]];
        // Manual calculation: -1*(-5*-9 - 6*8) - 2*(4*-9 - 6*-7) + (-3)*(4*8 - -5*-7)
        // = -1*(45 - 48) - 2*(-36 + 42) + (-3)*(32 - 35)
        // = -1*(-3) - 2*(6) + (-3)*(-3)
        // = 3 - 12 + 9 = 0
        assert_eq!(matrix_determinant(matrix), 0);
    }

    #[test]
    fn test_large_values() {
        let matrix = [[10, 20, 30], [40, 50, 60], [70, 80, 90]];
        // This matrix has linearly dependent rows, so determinant should be 0
        assert_eq!(matrix_determinant(matrix), 0);
    }
}
