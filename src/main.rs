use wonderful_pe::engine::Engine;

fn main() {
  let mut args = std::env::args();
  if args.len() != 2 {
    panic!("There must be an argument, which is the csv input path.")
  }
  let path = args.nth(1).unwrap();
  let mut engine: Engine = Default::default();
  engine.ingest_csv(&path).expect("Error processing input file");
}
