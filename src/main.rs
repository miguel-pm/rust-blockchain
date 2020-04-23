use blockchainlib::{ Hashable, Block, now };

fn main () {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 1, "hello world".to_owned(), difficulty);

    block.hash = block.hash();

    println!("{:?}", block);

    block.mine();
    println!("{:?}", block);
}
