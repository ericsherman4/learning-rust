mod rectangles;

fn main() {

    let mut tank1 = StorageTank {
        asset_name: String::from("Shell, EAST, 034"),
        diameter: 100f32,
        height: 20f32,
        are_stairs: false,
    };

    tank1.are_stairs = true;
    
    let _tank2 =  build_tank(&String::from("watertank"), 10f32, 5f32);
    let tank3 =  build_tank_efficiently("watertank", 10f32, 5f32);

    // equal sign will assign ownership of any heap allocated variables like strings.
    let tank4 = StorageTank {
        asset_name: String::from("oil tank"),
        ..tank3 //pull the rest of the fields from tank3
    };

    println!("{}", tank4.asset_name);

    println!("{}", tank3.asset_name);

    let _black = Color(0,0,0);
    println!("the color is {}, {}, {}", _black.0, _black.1, _black.2); 

    let _thing = AlwaysEqual;

    rectangles::run();


}

// tuple structs
struct Color(u8,u8,u8);

//unit like struct (struct with no fields)
struct AlwaysEqual;

// unlike tuples, rust can figure out which field you are borrowing from
// so you can have multiple mutable refernces at a time from a struct.
struct StorageTank{
    asset_name: String, //  we used the owned String type rather than the &str string slice type. 
    // This is a deliberate choice because we want each instance of this struct to own 
    // all of its data and for that data to be valid for as long as the entire struct is valid.
    are_stairs: bool,
    diameter: f32,
    height: f32,
}

fn build_tank(asset_name: &str, diameter: f32, height: f32) -> StorageTank
{
    StorageTank {
        asset_name: asset_name.to_string(),
        diameter: diameter,
        height:height,
        are_stairs: false,
    }
}

// field init shorthand, if param is the same name as field dont need to do field: param
fn build_tank_efficiently(asset_name: &str, diameter: f32, height: f32) -> StorageTank
{
    StorageTank {
        asset_name: asset_name.to_string(),
        diameter,
        height,
        are_stairs: false,
    }
}