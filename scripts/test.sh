# main.rs currently specifies no_main, which stops us running the integration tests
# so we replace it to run tests, then put it back

sed -i 's/\#\!\[cfg_attr(all(not(feature = \"export-abi\")), no_main, no_std)\]/fn main() {}/' src/main.rs
cargo test
sed -i 's/fn main() {}/#\!\[cfg_attr(all(not(feature = \"export-abi\")), no_main, no_std)\]/' src/main.rs

