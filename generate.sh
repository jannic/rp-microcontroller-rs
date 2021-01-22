set -e
cd pac/rp2040
svd2rust -i ../../svd/rp2040.svd
form -i lib.rs -o src/
rm lib.rs
cargo fmt
