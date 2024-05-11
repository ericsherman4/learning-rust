mod fib;


fn main() {
    let mut y = 5;
    println!("The value of x is: {y}");
    y = 6;
    println!("The value of x is: {y}");

    // const are always immutable, and they have to be uppercase
    // and they must be annotated (type defined)
    // must be defined at compile time and not run time.
    // consts can only be assigned constant expressions
    // let can only be defined in functions while consts can be defined at the global scope
    const Y_CONST: u32 = 4;

    // shadowing! 
    let x = 5;
    let x = x + 1; // creates a new variable x that shadows the previously defined x until this x goes out of scope
                    // or is shadowed by something else
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6

    // could then do 
    let mut x = x; // and now the variable is immutable.
    // so we could do some calculations on it and then for safety make it immutable after we done via shadowing
    // we could still reshadow this again with a mut or non-mut var

    // The first spaces variable is a string type and the second spaces variable
    // is a number type. Shadowing thus spares us from having to come up with
    // different names, such as spaces_str and spaces_num; instead, we can reuse 
    // the simpler spaces name.
    let spaces = "   ";
    let spaces = spaces.len();

    another_function(3, 'h');

    let is_even_num = is_even(3);
    println!("true or false {is_even_num}");

    // loops
    // YOU CAN RETURN VALUES FROM LOOPS
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter *2 //the semicolon here is optional. same for return statements.
        }
    };
    println!(" looky here! return value from loop! {result}");
    // ONE MAJOR CHANGE, YOU CAN SPECIFY WHICH LOOP TO BREAK OR CONTINUE WITH LOOP LABELS
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // no label so this will break out of inner loop
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    // rust also still has the normal while loops
    let mut number = 4;
    while number != 0 {
        number -= 1;
    }

    // python style for loops
    let array = [10,20,30,40,50];
    for element in array {
        // this doesnt require the index check during runtime so its faster. 
        println!("the value is {element}");
    }

    let fib_num = fib::fib_sequence(39);
    println!("fib num is {fib_num}");

    println!("you dont need a semi colon at the end of the last line in a function")

}

fn another_function(x : u32, y: char) {
    println!("Another function {x} and {y}");
}


// STATEMENTS DO NOT RETURN A VALUE
// EXPRESSIONS RETURN A VALUE
// IF YOU PUT A SEMICOLON AT THE END OF AN EXPRESSION, IT WONT RETURN A VALUE
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and
// specifying a value, but most functions return the last expression implicitly
fn is_even(num : u32) -> bool {
    num % 2 == 0
}

fn five() -> i32 {
    5
}

fn if_statements() {
    let number = 3;

    // rust will not auto convert integers to boolean types
    if number < 5 {
        println!("condition was true"); // this blocks are called arms just like match
    } else {
        println!("condition was false");
    }

    let number = if number > 10 { 0 } else { 10 };
}