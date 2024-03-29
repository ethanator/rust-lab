use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        for i in 0..v.len() - 1 - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1)
            }
        }
        println!("{:?}", v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        let sorted_v = vec![1, 3, 6, 4, 8, 11, 13];
        assert_eq!(v, sorted_v);
    }
}
