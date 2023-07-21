# Hands-on Task - Week 1

**Implement a basic program that uses ownership concepts**

## Task Details

In this task, students will create a simple Rust program that demonstrates the concepts of ownership, borrowing, and references. The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.

## Steps

1. Create a function called concatenate_strings that takes two string slices as arguments and returns a new String as the result of concatenating the two input strings.
2. Inside the concatenate_strings function, create a new String called result. Use the push_str() method to append the contents of the first input string slice, followed by the second input string slice.
3. Return the result string from the function.
4. Create two String variables, string1 and string2, and initialize them with appropriate values. string2 is a string slice (&str)
5. Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices).
6. Print the concatenated_string variable to the console.
7. Compile and run the program to ensure it works as expected.

## Checklist

1. Write the concatenate_strings function signature.
2. Implement the concatenate_strings function.
3. Initialize two String variables in the main function.
4. Call the concatenate_strings function with string slices of the variables.
5. Print the result to the console.
6. Compile and run the program to test its functionality.

## Expected Output


![Terminal output screenshot](/week_1/handson_task_week1/rust_program_output/terminal_output_screenshot.png)
