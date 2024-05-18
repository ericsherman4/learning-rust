mod enums;
mod coins;

fn main() {
    let _order1 = enums::SubwayRoll::SixIn;

    let order2 = enums::BetterSubwayRoll::TwelveIn(String::from("wheat"));

    println!("{:?}", order2);

    let m = enums::Message::Write(String::from("hello"));
    m.send();

    let nothing: Option<i32> = None;

    let x: i32 = 10;
    let y: Option<i32> = Some(5);
    // let sum =  x + y; doesnt work
    
    let sum = add_one(y);
    println!("{:?}", sum);

    coins::run();

    // default and catch alls
    let dice_roll = 9;
    match dice_roll {
        3 => println!("nice"),
        6 => println!("bad"),
        5 => (), // do nothing
        other_rolls => println!("you rolled {other_rolls}"),
    }

    // note that for strings, you might want to do "match &var".
    // that way when you are comparing in the arm, it wont bind it and remove ownership. 


    // can do this instead of an match statement
    let config_max = Some(3u8);
    // let config_max: Option<u8> = Option::None;
    
    // first comparison in match
    if let Some(max) = config_max {
        println!("the max is configured to be {}", max);
    }
    // same as _ case in match 
    else { // dont need the if statement
        println!("it is none");
    }
}


fn add_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // x is binded to the variant Some
        // we name the local variable i
        Some(i) => Some(i + 1)
    }
}
