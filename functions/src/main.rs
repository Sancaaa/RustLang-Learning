
fn main() {
    println!("Hello, world!");
    // state_xp_express(5);
    let x = test_return(6);
    println!("{x}");
}

fn state_xp_express (x: i32){
    let y = {
        x + 1
    };
    println!("{y}");
}

fn test_return(x : i32) -> i32 {
    x + 5;
}