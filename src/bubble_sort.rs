fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let mut v = vec![4, 6, 1, 8, 11, 13];
        let want = vec![1, 4, 6, 8, 11, 13];
        super::bubble_sort(&mut v);
        assert_eq!(want, v);
    }
}
