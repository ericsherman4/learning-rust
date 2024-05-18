pub enum SubwayRoll {
    SixIn,
    TwelveIn,
}

#[derive(Debug)]
pub enum BetterSubwayRoll {
    SixIn(String),
    TwelveIn(String),
}


// why not just make a struct? function signatures are easier (only need to pass in a Message)
pub enum Message {
    Quit,
    Move { x: i32, y: i32 }, 
    Write(String),
    ChangeColor(i32, i32, i32),
}

// can make messages on enums

impl Message {
    pub fn send(&self) {
        println!("hi");
    }
}