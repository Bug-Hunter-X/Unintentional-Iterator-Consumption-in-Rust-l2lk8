fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let iter = vec.iter();
    let mut iter2 = vec.iter();
    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter2.next().unwrap());
}
