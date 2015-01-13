extern crate "rustc-serialize" as serialize;

use serialize::hex::FromHex;
use serialize::base64::{ToBase64, STANDARD};

fn main() {
    let base16 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let converted = base16.from_hex().unwrap().to_base64(STANDARD);

    println!("Hex:       {}\nBase64:    {}\nConverted: {}", base16, base64, converted);
}

