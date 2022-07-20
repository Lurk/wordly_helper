#sh
rm -rf ./release
cargo build --release
mkdir "release"
cp ./target/release/wordle_api ./release/wordle_api
cp -R ./data/ ./release/data
