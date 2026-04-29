use std::collections::HashMap;

fn main() {
    let mut strings = HashMap::new();
    
    strings.insert(String::from("banjo"), 5);
    strings.insert(String::from("mandolin"), 8);
    strings.insert(String::from("fiddle"), 4);

    strings.insert(String::from("banjo"), 4);

    let instrument = String::from("fiddle");
    let string_count = strings.get(&instrument).copied().unwrap_or(0);

    println!("A {} has {} strings", instrument, string_count);
}
