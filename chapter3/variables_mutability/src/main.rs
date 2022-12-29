use std::cmp::Ordering;

fn main() {
    // mutability
    let x = 5;
    let ptr_1 = &x;
    let x = 6;
    let ptr_2 = &x;
    let value_2 = *ptr_2;
    println!("The value of the first x is: {ptr_1}, at {:p}",ptr_1);
    println!("The value of the second x is: {ptr_2}, at {:p}",ptr_2);
    println!("the value stored at {:p} is {:?}",ptr_2,value_2);
    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let ptr_three_hours_in_seconds = &THREE_HOURS_IN_SECONDS;
    let mut constant_copy = THREE_HOURS_IN_SECONDS;
    println!("The value of constant 'THREE_HOURS_IN_SECONDS' is {THREE_HOURS_IN_SECONDS}, at {:p}", ptr_three_hours_in_seconds);
    println!("Multipled by 2: {}",THREE_HOURS_IN_SECONDS*2); 
    constant_copy+=1;
    println!("constant + 1: {}", constant_copy);
    // shadowing
    let xs = 5;
    println!("The value of the first xs is: {xs}, at {:p}",&xs);
    let xs = xs + 1;
    println!("The value of the 2nd xs is: {xs}, at {:p}",&xs);
    {
        let xs = xs * 2;
        println!("The value of xs in the inner scope is: {xs}, at {:p}",&xs);
    }
    println!("The value of xs after the inner scope is: {xs}, at {:p}",&xs);
    // you cant use shadows inside the loop as the scope will be reinitialized each iteration, the solution is to use mutable variable manipulation
    let mut x_shadow = 1;
    const BIG_INTEGER: i64 = 100000;
    loop {
        x_shadow = x_shadow * 2;

        match x_shadow.cmp(&BIG_INTEGER) {
            Ordering::Greater => {
                println!("Condition met: {}, at {:p}",x_shadow,&x_shadow);
                break;
            },
            Ordering::Less => {
                //let x_shadow = x_shadow * 2;
                println!("Value is now at: {}, at {:p}",x_shadow, &x_shadow);
            },
            Ordering::Equal => {
                //let x_shadow = x_shadow + 1;
                println!("Value is now at: {}, at {:p}",x_shadow,&x_shadow);
            },

        }

    }
    // shadow example, by using shadowing we can ignore potential variable type errors
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Length of spaces: {spaces}");
    // convert the variable type back to string so that we can mutate the original spaces2
    let mut spaces2 = "   ";
    let binding = spaces2.len().to_string();
    spaces2 = &binding;
    println!("Length of spaces2: {spaces2}");

}



