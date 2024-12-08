fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {}", iter.next().unwrap());
    // This will panic because the iterator is already consumed!
    println!("Second element: {}", iter.next().unwrap());
}