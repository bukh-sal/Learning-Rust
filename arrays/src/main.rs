fn main() {
    let my_arr:[i8; 4] = [1, 2, 3, 4];
    println!("Array (Has one dtype only), my_arr = {:?}", my_arr);
    
    let my_tuple: (i8, i32, i64, &str, bool) = (1, 2^31 - 1, 2^32, "Salman", true);
    println!("Tuple (Has many dtypes), my_tuple = {:?}", my_tuple);

}
