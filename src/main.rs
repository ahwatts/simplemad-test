use std::fs::File;

use simplemad::Decoder;

fn main() {
    let file = File::open("hit_somebody.mp3").expect("Unable to open file");
    let decoder_rslt = Decoder::decode(file);
    if decoder_rslt.is_ok() {
        println!("Decoder is ok");
    } else {
        println!("Decoder is not ok: {:?}", decoder_rslt.err());
    }
}
