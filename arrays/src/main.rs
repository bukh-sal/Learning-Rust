fn print_arr(arr: [i8; 10]){
    let mut pad_size = 0;
    for i in arr {
        println!("{:>pad_size$}", i);
        pad_size += 1;
    }
}


fn main() {
    let my_arr:[i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Array (Has one dtype only), my_arr = {:?}", my_arr);
    
    let my_tuple: (i8, i32, i64, &str, bool) = (1, 2^31 - 1, 2^32, "Salman", true);
    println!("Tuple (Has many dtypes), my_tuple = {:?}", my_tuple);

    let my_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_arr(my_arr);

    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("my_vec = {:?}", my_vec);

    println!("my_arr == my_arr -> {}", my_vec == my_arr);

}
