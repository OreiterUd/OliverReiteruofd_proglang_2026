#[derive(Debug)]
enum List {
    Pair(i32, Box<List>),
    Nil,
}

fn head(list: List) -> Option<i32> {
    match list {
        Pair(num, _) => Some(num),
        _ => None
    }
}

fn tail(list: List) -> Option<Box<List>> {
    match list {
        Pair(_, out) => Some(out),
        _ => None
    }
}

use crate::List::{Pair,Nil};

fn main() {

    let list = Pair(1, 
                Box::new(Pair(2,
                    Box::new(Pair(3,
                        Box::new(Nil))))));

    println!("{:?}", list);
    println!("{:?}", crate::head(list));
}
