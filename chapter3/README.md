# Chapter 3

Key concepts
Mutability
- variables created by using 'let x=0'
- Mutable variables 'let mut x=0' so that we can modify the value of x, else error will be thrown or we can do another 'let x=1'
- A new pointer address will be assigned to the re-declared variable.
- Read pointers by using &x , println!("{:p}", ptr_x)
- Read value by using *ptr_x.
- Constants can only be defined once, and use all caps as a best practice.
- shadowed variables will be reference within the defined scoped, once we exit the inner scope, the variable will reference back to the previous address.
- We cannot use shadowed variables within a loop, each loop can be considered as its own scope, cannot stack. But we can still update the value inside mutable variables.
- Mutable variables cannot change type from str to integer.

Datatypes
- floats f64, f32
- simple arithmatic such as +-/* , nothing new with floats and integers.
- Copy paste how to print the type of a variable: https://www.includehelp.com/rust/print-the-type-of-variables.aspx , by default the value will be assigned a i32, if divisible, else a f64.
- bool is pretty standard,
- char is declared with single quotes, as it can fit anything that is 4 bytes in size, it can accept more than ASCII.
- string literals can be considered as a vector of char.
- Tuples has a fixed length that cannot be changed. Each position in tuple has a type that obeys the same mutability in 3.1, require to declare mutable tuple to change the values.
- Array must have the same type through elements.
- Array useful for stack allocation instead of heap, where the length of array is fixed. If unsure, use vector.

Functions
- standard way of calling functions in general. Removing the semicolon will evalute the value of the last variable and you are able to return a value. 

Comments
- Nothing to add.

Control Flow
- if : must use bool type, integer will throw error.
- if : Can use if expression in a let statement.
- loop : new concept of loop labels to break out of the desired loop.
- while : Nothing new.
- for : nothing new.
