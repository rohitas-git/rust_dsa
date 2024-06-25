
fn find_zigzag_sequence(mut vector: Vec<u32>, size: usize) {
    vector.sort();
    let mid = (size - 1) / 2;

    let mut start = mid;
    let mut end = size - 1;
    while start <= end {
        (vector[start], vector[end]) = (vector[end], vector[start]);
        start += 1;
        end -= 1;
    }

    println!("{vector:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vector = vec![1,2,3,4,5,6,7];
        let size = vector.len();
        find_zigzag_sequence(vector, size);
    }
}
