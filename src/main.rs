use blockchainlib::{ Block, now };

fn main () {
    let block = Block::new(0, now(), vec![0; 32], 0, "hello world".to_owned());
    println!("{:?}", &block);
}
