pub mod max_pairwise_product;
pub mod ask;
use std::process;

fn main() {
   let result = max_pairwise_product::run().unwrap_or_else(|err| {
      eprintln!("Problem with max_pairwise_product call: {}", err);
      process::exit(1);
   });

   println!("{}", result);
}

