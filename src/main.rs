mod hash_map;

fn main() {
    let mut a: hash_map::Map<String, usize> = hash_map::Map::new();


    println!("{:?}", a);

    let res = a.remove("Bob".to_string());

    println!("{:?}", res);

    println!("{:?}", a);
    
}