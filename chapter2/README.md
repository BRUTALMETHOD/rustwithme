# Chapter 2 Notes
Make a guessing game.

Using std::io library to read input into a mutable string location and throw error if fails.
the expect function works similar to the unwrap function, either return the value or panics with the given msg. --No idea at this point how it works together with the buffers read.--
read_line returns a Result that has the expect method. If this Result instance is of an Ok value, it will return the value. If the expect method is not used, the compiler will prompt us that the Result instance is not handled for the Err value.

The match operator works like a case operator, the trim method removes leading and trailing whitespaces, extremely useful with io inputs.
In the loop scenario, we are required to create a new String variable each time we are expected to do the read_line. Suspect it is something related to the seek(0) of the buffer.
We are able to run match operator directly on the Result instance given by the parse method available for all strings.

Require to be familiar with the standard macros and libraries available for performing simple tasks, need to remind ownself not to reinvent the wheel.
