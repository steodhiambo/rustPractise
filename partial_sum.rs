pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(arr.len() + 1);
    let mut sum = 0;
    
    for &num in arr.iter().rev() {
        sum += num;
        result.push(sum);
    }
    
    result.push(0);
    
    result.reverse();
    result
}