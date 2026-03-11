use std::io;

fn main() { 
    println!("=====Hello This My Calc=====");
    println!("=====press q for quit=====");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let num: i32 = input.trim().parse().expect("This don't number");

        if input.trim() == "q" {
            println!("Bay!");
            break;
        }

        input.clear();

        io::stdin().read_line(&mut input).expect("Error");
        let num1: i32 = input.trim().parse().expect("This don't number");

        println!("{} + {} = {}", num, num1, num + num1);

        
    }
}
