# Chapter 3

Key concepts
- variables created by using 'let x=0'
- Mutable variables 'let mut x=0' so that we can modify the value of x, else error will be thrown or we can do another 'let x=1'
- A new pointer address will be assigned to the re-declared variable.
- Read pointers by using &x , println!("{:p}", ptr_x)
- Read value by using *ptr_x.
- Constants can only be defined once, and use all caps as a best practice.
- shadowed variables will be reference within the defined scoped, once we exit the inner scope, the variable will reference back to the previous address.
- We cannot use shadowed variables within a loop, each loop can be considered as its own scope, cannot stack. But we can still update the value inside mutable variables.
- Mutable variables cannot change type from str to integer.

