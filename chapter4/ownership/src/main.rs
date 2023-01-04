
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    let add_s = &s;
    //s.push_str(", world!"); // This statement will fail due to borrow issue with add_s.
    println!("{:p}", add_s); // This will print `hello, world!`
    println!("Hello, world!");
    // variables that have fixed sizes are stored onto the stack and able to be copies without s1 invalidated by rust.
    let mut s1 = 5;
    let s2 = s1;
    println!("s1 = {}, s2 = {}",s1,s2);
    // testing of borrowing with functions
    let fn_s = String::from("");
    let fn_s = take_ownership(fn_s);
    // at this point fn_s is no longer valid as the value has been moved to take_ownership
    println!("{fn_s}");

}

// modifications done to the function in order to pass the value back to the main function if the value is still required.
// Otherwise we can stop at println, rust really enforce good practices.
fn take_ownership(some_string: String) -> String {
    println!("some_string: {some_string}");
    return some_string;
}
