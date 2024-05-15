
pub fn hello_world_test() {
    println!("hello world from slices");
}

// a small programming problem: write a function that takes a string of words 
// separated by spaces and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, the whole string 
// must be one word, so the entire string should be returned.
// passed by reference so this function doesnt take ownership of the string
pub fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

//slices are special kinds of references because they are fat pointers (poitners with metadata)
// STRINGS ARE VECTORS OF BYTES U8 THAT CONTAINS A LENGTH AND A BUFFER THAT HAS A PTR AND A CAPACITY
pub fn first_word_slice_1(s: &String) -> &str { // STRING SLICES ARE str (so are string literals)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] //deref coercions
}

// here we can pass in a slice or a String or even a string literal
pub fn first_word_slice_2(s: &str) -> &str { 
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}