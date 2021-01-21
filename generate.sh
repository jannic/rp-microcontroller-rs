set -e
svd2rust -i rp2040.svd
form -i lib.rs -o src/
rm lib.rs
cargo fmt
