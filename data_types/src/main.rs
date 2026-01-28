use std::io;

fn main(){
    let a = [1,2,3,4,5];

    println!("Enter an array index");

    let mut index= String::new();

    io::stdin().read_line(&mut index).expect("failed");

    let index: usize = index.trim().parse().expect("not a num");

    let element = a[index];

    println!("{element}");


}
    