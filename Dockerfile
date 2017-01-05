FROM ctarwater/armhf-alpine-rpi-glibc:latest

COPY /target/armv7-unknown-linux-gnueabihf/debug/examples/main /rust-main
CMD /rust-main

