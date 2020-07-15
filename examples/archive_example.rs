use installer::archive::*;

fn main(){
    let args: Vec<String> = std::env::args().skip(1).collect();
    let archive = archive(&args.get(0).unwrap()).unwrap();
    println!("archived with len: {}", archive.len());
    //let compressed = compress(&archive).unwrap();
    //println!("compressed");
    //let decompressed = decompress(&compressed).unwrap();
    //println!("decompressed");
    dearchive(&archive, "./random").unwrap();
    println!("dearchived");
}