#CARGO_TARGET_DIR=../run/artifacts cargo build --example leds --target=armv7-unknown-linux-gnueabihf --verbose

cross build --example main --target=armv7-unknown-linux-gnueabihf
rdt push --force-build -s . resin.local #10.42.0.71