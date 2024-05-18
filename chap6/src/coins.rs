
pub fn run() {
    println!("you have {}", get_value_cents(Coin::Quarter(State::NV)))
}

enum State {
    NJ,
    CA,
    NY,
    NV, 

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
    Dollar,
}

fn get_value_cents(coin : Coin) -> u8 {
    match coin {
        // mutliline arm
        Coin::Penny => {
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // not doing anything with the state, but we could print it
        Coin::Quarter(state) => {
            println!("This state is fun");
            25
        }
        Coin::Dollar => 100,
    }
}
