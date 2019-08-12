use std::env;
use morse::io;
use morse::waveform;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: morse <out_file> <words...>")
    } else {
        let sentence = &args[2..].join(" ");
        let wav = waveform::gen_waveform(&sentence, 10, 8);
        io::write_wav(&args[1], &wav);
    }
}
