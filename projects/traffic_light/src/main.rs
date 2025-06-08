enum TrafLight {
    Red,
    Yellow,
    Green,
}

fn main(){
    let indicator = TrafLight::Red;

    match indicator{
        TrafLight::Red => println!("Stop! The light is red."),
        TrafLight::Yellow => println!("Ready to Go! The light is yellow."),
        TrafLight::Green => println!("Go! The light is green."),
    }
}