use std::iter::zip;

pub fn contains_duplicates<T: Eq>(vec: &Vec<T>) -> bool {
    for (i,a) in zip(0.., vec) {
        for b in &vec[i + 1..] {
            if a == b {
                return true;
            }            
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicates_positive() {
        assert!(contains_duplicates(&vec![1, 2, 3, 2]));
    }

    #[test]
    fn contains_duplicates_negative() {
        assert!(!contains_duplicates(&vec![1, 2, 3, 4]));
    }
}