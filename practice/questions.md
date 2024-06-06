Sure da! Here are 10 tasks with one big project for you to practice the basics section. These tasks will help you solidify your understanding and give you hands-on experience.

### 10 Tasks

1. **Temperature Converter**: Convert temperatures between Celsius and Fahrenheit.
    - Requirements:
        - User inputs temperature and conversion type (C to F or F to C).
        - Display the converted temperature.

2. **Simple Calculator**: Perform basic arithmetic operations.
    - Requirements:
        - User inputs two numbers and the operation type (add, subtract, multiply, divide).
        - Display the result.
        - Handle division by zero.

3. **Guess the Number Game**: Create a game where the user guesses a randomly generated number.
    - Requirements:
        - Generate a random number between 1 and 100.
        - User inputs guesses until they get it right.
        - Provide hints (too high, too low).

4. **Reverse a String**: Reverse a user-provided string.
    - Requirements:
        - User inputs a string.
        - Display the reversed string.

5. **Palindrome Checker**: Check if a given string is a palindrome.
    - Requirements:
        - User inputs a string.
        - Check if the string is the same forwards and backwards.
        - Ignore case and spaces.

6. **Factorial Calculator**: Calculate the factorial of a number.
    - Requirements:
        - User inputs a number.
        - Calculate and display the factorial of the number.

7. **Fibonacci Sequence Generator**: Generate the first N numbers in the Fibonacci sequence.
    - Requirements:
        - User inputs the number of terms.
        - Display the Fibonacci sequence up to the given number of terms.

8. **Prime Number Checker**: Check if a given number is prime.
    - Requirements:
        - User inputs a number.
        - Check and display if the number is prime.

9. **Word Count**: Count the number of words in a given sentence.
    - Requirements:
        - User inputs a sentence.
        - Count and display the number of words in the sentence.

10. **Basic Banking System**: Simulate a basic banking system with deposit and withdrawal functionalities.
    - Requirements:
        - User can deposit and withdraw money.
        - Display the current balance.
        - Handle edge cases like withdrawing more than the balance.

### Big Project: Contact Management System

#### Objective:
Create a simple contact management system that allows users to add, view, update, and delete contacts.

#### Requirements:
1. **Add Contact**:
    - User inputs contact details (name, phone number, email).
    - Store the contact in a list.

2. **View All Contacts**:
    - Display all contacts in the list.

3. **Update Contact**:
    - User inputs the contact name to update.
    - Allow user to update contact details.

4. **Delete Contact**:
    - User inputs the contact name to delete.
    - Remove the contact from the list.

5. **Search Contact**:
    - User inputs the contact name to search.
    - Display the contact details if found.

6. **Exit**:
    - Allow user to exit the application.

#### Implementation Tips:
- Use structs to define contact details.
- Use vectors to store contacts.
- Use functions to modularize different operations (add, view, update, delete, search).
- Handle input errors and edge cases gracefully.

### Sample Code Structure:

```rust
use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("Contact Management System:");
        println!("1. Add Contact");
        println