extern crate flate2;
use flate2::Compression;
use flate2:: write:: GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

fn main(){

    if args().len() != 3{
        eprint!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    let output = File::create(args().net(2)).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input,&mut encoder).unwrap();
    let outut = encoder.finish().unwrap();
    println!(
        "source len: {:?}", input.get_ref().metadata.unwrap.len()
    )

}