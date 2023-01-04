fn TypeOf<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    println!("Hello, world!");
    #[warn(unused_variables)]
    // list of all datatypes
    //floats
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let quotient2 = 4 / 2 ;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
    TypeOf(&x);
    TypeOf(&sum);
    TypeOf(&difference);
    TypeOf(&quotient);
    TypeOf(&quotient2);
    TypeOf(&truncated);
    // booleans
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t},{f}");
    // char , more than just ASCII, declared with single quotes where strings literals are with double quotes
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c},{z},{heart_eyed_cat}");
    // tuples
    let mut x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    x.2 = 2;
    println!("{},{},{},{}",one,six_point_four,five_hundred,x.2);
    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3;5]; 
    println!("{a:?}");
    println!("{a2:?}");
}

