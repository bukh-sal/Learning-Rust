fn main() {
    let mut my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    println!("my_num {}", my_num);
    
    println!("my_num_ptr {:?}", my_num_ptr);
    
    my_num = 25;
    // it will not run wihtout unsafe because "raw pointers may be null"
    unsafe {
        println!("val from ptr {:?}", &*my_num_ptr);
    }
    
    let num_val: i32 = my_num;
    println!("num_val {}", num_val);

    let mut my_speed: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;
    println!("my_speed {}", my_speed);
    println!("my_speed_ptr {:?}", my_speed_ptr);
}
