// takes arrary regardless of size and data type
fn display_array<T>(arr: &[T]) where T: std::fmt::Display {
    print!("[");
    for i in 0..arr.len() {
        print!("{}", arr[i]);
        if i != arr.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}


fn main(){

    let is_active:bool = true;
    println!("user is active {}", is_active);

    let is_user = true;
    println!("inferred type = {}", is_user);

    let my_arr = [1, 2, 3];
    println!("my_arr[0] = {}", my_arr[0]);

    // from 0 to 100
    let mut lg_arr:[i32; 100] = [0; 100];
    for i in 0..lg_arr.len() {
        lg_arr[i] = i as i32;
    }

    println!("last element {}", lg_arr[lg_arr.len() - 1]);

    display_array(&lg_arr);
}


