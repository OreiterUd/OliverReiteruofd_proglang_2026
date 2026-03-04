fn main() {
    let mut v = Vec::new();
    v.push(42);
    v.push(67);
    v.push(23);
    v.push(1723);



    let b = v.first();
    println!("{:?}", b);
    v.reverse();
    println!("{:?}", v);

}
