rm -Rf target
RUSTFLAGS="-Zunstable-options -Cpanic=immediate-abort" cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size" \
  --target x86_64-unknown-linux-gnu --release
# TODO
# Change from x86_64-unknown-linux-gnu to something more cross platform
