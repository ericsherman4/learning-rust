
pub fn run() {

    let rect = Rectangle { width: 10, height: 5};

    println!("area is {}", get_area(&rect));
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);

    /* prints out
    area is 50
    rect is Rectangle { width: 10, height: 5 }
    rect is Rectangle {
        width: 10,
        height: 5,
    }
     */

    dbg!(&rect); // note prints to stderr instead of stdout
    /* prints out: 
    [src\rectangles.rs:20:5] &rect = Rectangle {
        width: 10,
        height: 5,
    }
     */

    // can also do this
    let rect2 = Rectangle {
        width: dbg!(100 * 10),
        height: 3,
    };
    println!("rect is {:?}", rect2);

    println!("rect area using struct method is {}", rect.area());
    println!("rect can hold rect2 {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(5);
    dbg!(rect3);

}


#[derive(Debug)] // allows us to use :? and :#? when printing
struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle {
    // &self short for self:&self
    // we are borroeing immutably
    // to change instance need &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }

    // associated function
    // capital Self are aliases for the type after impl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // there is no constructor keyword but most people use "new"


}

fn get_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}