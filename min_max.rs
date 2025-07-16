pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    if nb_3 == i32::MIN {
        // Two-input case
        let min = nb_1.min(nb_2);
        let max = nb_1.max(nb_2);
        (min, max)
    } else {
        // Three-input case
        let min = nb_1.min(nb_2).min(nb_3);
        let max = nb_1.max(nb_2).max(nb_3);
        (min, max)
    }
}

// Wrapper for two arguments
pub fn min_and_max_two(nb_1: i32, nb_2: i32) -> (i32, i32) {
    min_and_max(nb_1, nb_2, i32::MIN)
}

fn main() {
    println!(
        "Minimum and maximum are: {:?}",
        min_and_max_two(2, 4) // Use the two-argument wrapper
    );
}