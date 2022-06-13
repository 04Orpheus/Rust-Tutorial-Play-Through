use std::io;

fn main() {
    let x = 5;

    let x = x+1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let floored = 2/3;
    println!("{}",floored);

    let floored = 2.0/3.0;
    println!("{}", floored);


    let remiander = 43 % 5;
    println!("{}", remiander);

    let c = "ℹ️";

    println!("{}", c);

    let t = true;
    let f:bool = false;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a:[i32;5] = [1,2,3,4,5];

    let a = [3;5]; // [3,3,3,3,3]
    let first = a[0];
    let second = a[1];

    let a = [1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
