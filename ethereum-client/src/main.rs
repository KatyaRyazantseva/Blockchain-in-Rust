mod block;
fn main() {
    let mut blockchain = block::Blockchain::new();
    blockchain.add_block("First block".to_string()).unwrap();
    blockchain.add_block("Second block".to_string()).unwrap();

}