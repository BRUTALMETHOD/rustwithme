#Chapter 4 Ownership
- Stack
-- last in, first out
-- Known fixed size
- Heap
-- Less organized, unknown size on compile should be stored in heap.

Ownership rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

First example of ownership was discussed under chapter3 mutability with the shadowing example.
When variables goes out of scope, Rust calls the drop function automatically to return the memory to the free heap space.
When we 'copy' variables by doing s1 = s2; , the stack data is copies but the heap data remains the same. You will have s1 and s2, with both pointer at the same address.
To perform deep copy, can use s1.clone() method.
Variables that has a known size can be stored fully to the stack. E.g. assignment of a integer.
such cases are not affected by double free errors that occurs as the heap memory is freed when multiple variables goes out of scope.
Rust has a Copy trait that can use to annotate the types that are stored in the stack. You cant annotate a type with a Copy trait if it has a Drop trait.
More about traits in chapter 10.


- capacity vs length in a variable to be reviewed in the future.
