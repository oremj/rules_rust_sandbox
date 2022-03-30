use ahash::AHashMap;



fn main() {
    let mut map: AHashMap<i32, i32> = AHashMap::with_capacity(4);
    map.insert(12, 34);
    map.insert(56, 78);
    println!("Hello, world!");
}
