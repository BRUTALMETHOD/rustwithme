fn main() {
    fn mini(){
        println!("Mini function");
    }
    println!("Hello, world!");
    mini();
    another_function();
    let y = {
        let x = 3;
        let y = 2;
        let z = 1;
        x + 1;
        z
    };
    println!("The value of y is: {y}");
}

fn another_function() {
    println!("Another function.");
}

