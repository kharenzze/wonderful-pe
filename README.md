# Welcome to Wonderful-pe

This is a simple payment engine built in Rust. 

## How to use 

You need to set up a Rust development environment. Check [this](https://www.rust-lang.org/learn/get-started) in order to set it up.

Then, you can run the app this way:

```sh
cargo run -- ./tests/samples/little.csv

#if you want to save the output to a .csv file
cargo run -- ./tests/samples/little.csv > output.csv

#You can enable logger by setting RUST_LOG env var
RUST_LOG='info'
cargo run -- ./tests/samples/little.csv
```


## Data

Sample input

```csv
client,tx,amount,type
1,1,10.0,deposit
2,2,2.0,deposit
1,3,10.0,deposit
1,5,10.0,deposit
1,4,1.5,withdrawal
2,5,3.0,withdrawal
1,3,,dispute
1,5,,dispute
1,3,,resolve
1,5,,chargeback
```

Sample output

```csv
client,available,held,total,locked
2,2.0,0.0,2.0,false
1,18.5,0.0,18.5,true
```

## Implementation info

The code is splitted in a lib (lib.rs) and an executable (main.rs), as suggested in [The Rust Programming Language guide](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects)

### Money amounts

* Represented as an u64, wrapped in Amount struct in order to define custom methods like serialiation and deserialization.
* Handles 4 decimal digits. Max number is 2^64 / 10^4.
* Does not handle negative numbers. Transactions that generate negative numbers are rejected.
* Repr: 23.05 ==> 230500_u64

### External libraries

* csv and serde to handle de/serialization of dsv data.
* log and env_logger to handle logs
* thiserror to quickly create custom error types with macros, instead of implementing verbose Error trait manually

### Tests

* Sample input csv files can be found in ./tests/sample/
* Some unit tests can be found in files 

### Ideas

* Wrap the engine in using actix::Actor, so that you can read different csv inputs in different threads and a single actor controls the engine.
* Move Hashmap to [DashMap](https://github.com/xacrimon/dashmap), so that transactions can can be applied in parallel.