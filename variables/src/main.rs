// fn main() {
//     let x = 5;
//     println!("the value of x is: {x}");
//     let x = 6;
//     println!("the value of x is: {x}");
// }

//shadowing example below
fn main() {
    let x = 5;

    {
        let x = x * 2;
        println!("the value of the x in the inner scope is {x}")
    }

    println!("the valus of the x is {x}")
}