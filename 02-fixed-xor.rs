extern crate "rustc-serialize" as serialize;

use serialize::hex::{FromHex, ToHex};

fn main() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    let output:Vec<u8> = input1.from_hex().unwrap().iter().zip(
        input2.from_hex().unwrap().iter()
    ).map(|(&a, &b)| a ^ b).collect();

    println!("Input 1:   {}\nInput 2:   {}\nExpected:  {}\nConverted: {}", input1, input2, expected, output.to_hex());
}

