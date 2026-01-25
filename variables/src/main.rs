fn main() {
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("value of x is {x}");    
    }

    println!("value of x is {x}");

    let spaces = "sanca";
    println!("value of spaces is {spaces}");
    let spaces = spaces.len();
    println!("value of spaces is {spaces}");

    // let mut space = "santa";
    // println!("value of space is {space}");
    // // space = space.len();
    // println!("value of space is {space}");
}
