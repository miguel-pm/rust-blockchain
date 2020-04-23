use blockchainlib::{ Hashable, Block, now };

fn main () {
    let mut block = Block::new(0, now(), vec![0; 32], 1, "hello world".to_owned());

    let hash = block.hash();
    block.hash = hash;

    println!("{:?}", block);
}
