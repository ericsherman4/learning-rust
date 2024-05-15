mod slices;

fn main() {

    try_moving();

    let string1 = String::from("hello");
    let string2 = String::from("world");
    try_borrow(&string1, &string2);
    println!("{string1} and {string2}");

    try_deference();

    // vec! is a macro to create a vector
    // Pointer Safety Principle: data should never be aliased and mutated at the same time.
    // aliased is for example you have a vector and then you have a reference to an element of the vector and
    // then you push an element onto the vector which has to the potential to re-allocate the arary which 
    // would then invalidate the reference. but anyway this is aliasing and the changing of the array is mutating.

    // DATA CAN ONLY BE ACCESSED THROUGH THE OWNER. 
    // references are non-owning pointers and need different rules to ensure pointer safety principle
    // whole point of references are to create aliases

    mutable_references();

    slices::hello_world_test();

    // this overview is actually really good as a summary https://rust-book.cs.brown.edu/ch04-05-ownership-recap.html

}


fn try_moving() {
    let basic = String::from("hello"); // create string from literal
    
    // this line will borrow from basic and give it to in_str in the function
    let full = add_world(basic); 
    println!("{full}");

    //Moved heap data principle: if a variable x moves ownership of heap data to another variable y, 
    // then x cannot be used after the move.
    // no worky.
    // you can fix this using .clone() to make a deep copy of basic.
    // println!("{full} and {basic}");
    
    // RUST DEALLOCATES HEAP DATE ONCE THE OWNER GOES OUT OF SCOPE
}

fn add_world(mut in_str : String) -> String {
    in_str.push_str(" world");
    in_str
    
}

fn try_borrow(s1: &String, s2: &String) {
    println!("from inside dont_borrow: {s1} and {s2}");
}

fn try_deference() {
    let mut x: Box<u32> = Box::new(1);
    let _a = *x;
    *x +=1;
    let r1: &Box<u32> = &x; // points to x on the stack
    let _b : u32 = **r1;

    let r2 : &u32 = &*x; // points directly to the heap value for x.
    let _c = *r2;
}

fn mutable_references() 
{
    // Mutable References Provide Unique and Non-Owning Access to Data
    let mut v: Vec<i32> = vec![1,2,3];

    // let num: &i32 = &v[2];
    // *num += 1; cant do this because youre borrowing. 
    // so num owns v[2] but cannot write to it because we didn't do let mut num.
    // NOTE that THESE PERMISSIONS are for reference variable.
    // but also *num does not own v[2] and therefore cant write to it only read.

    // but you can make a mutable reference
    // *num can be read and modified.  but doesnt own.
    // num owns v[2] and can read.
    let num: &mut i32 = &mut v[2]; // mutable reference.
    *num +=1;
    // with mutable reference you now CAN NO LONGER READ from v[2] until num goes out of use. 
    // num owns and can read. you cant write because there is no mut on num, cant assign it something else.
    // and *num doesnt own but can read and write.

    // can downgrade to a ready only reference
    let _num2: &i32 = &*num; // this is a reference to the number itself.
    // this downgrades num to just be a read only reference now. 
    // num2 took ownership of v[2] from num. 

    // ----------------------

    // permissions are returned at the end of a references lifetime.
    // as soon as the reference variable stops being used, permissions are returned. 
    // EXAMPLE:
    let mut _x = 1; // x can read write and own
    let y = &_x; // y has ownership now and can read. *y can read.
    // x now loses owernship and write permissions.
    // y* cant write because its not a mutable reference.
    let z = *y; // so z gets the value of x
    // after this line is run y is no longer used. so y an *y lose everything
    // z gets read read and ownership since the it was just copying an int.
    // z not writable because mut x was not used.
    // x gets read, write, own because y is dead.
    _x += z; 
    // after this z and x lose everything (r,w,o)

    /*
     Data Must Outlive All Of Its References!!!!!
     Can't have the data go away before the reference is dies. 
     kind of like allocating a pointer in a function and then returning the pointer
     For now, it's enough to know that: (1) input/output references are treated 
     differently than references within a function body, and (2) Rust uses a 
     different mechanism, the F permission, to check the safety of those references.
     */

    // ALSO NOTE: IT IS ILLEGAL TO USE A MUTABLE REFERENCE TO A VALUE WHILE AN IMMUTABLE REFERENCE TO 
    // A VALUE IS LIVE

}

fn string_that_lives_forever() -> &'static str {
    "hello world"
}

// REMEMBER A STRING AND INT ARE NOT THE SAME, A STRING ON ITS OWN IS A REFERNECE
// let name = String::from("Ferris");
// fn award_phd(name: &String) {
//     let mut name = *name; // wont compile because of a shared reference but assuming it did compile...
//     name.push_str(", Ph.D.");
// }
// The statement let mut name = *name makes name take ownership of the input string. 
// BUT THE CALLER STILL RETIANS OWNERSHIP OF THE STRING.
// SO WE AFTER AWARD_PHD, STRING WILL BE DECALLOCED
