fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // in loop, the ownership of iter will be token away
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("The is: {}", total);

    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

    }

    let v1: Vec<i32> = vec![1, 2, 3];

    
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);


    iterator_demonstration();
}
