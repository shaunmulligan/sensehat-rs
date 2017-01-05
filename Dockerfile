FROM ctarwater/armhf-alpine-rpi-glibc:latest

# [insert additional runtime requirements here]

COPY /target/armv7-unknown-linux-gnueabihf/debug/examples/ /examples/
CMD /examples/humidity

