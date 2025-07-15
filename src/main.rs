use crate::hex::Hex;

mod hex;

fn main() {

    let hex = Hex::color_from_str("I_Is_Color_");
    println!("https://www.color-hex.com/color/{}", hex);

    let hex = Hex::color_from_str("I_Is_Color_");
    println!("https://www.color-hex.com/color/{}", hex);
    
}
