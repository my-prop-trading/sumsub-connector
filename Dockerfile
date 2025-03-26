FROM ubuntu:24.04
COPY ./target/release/sumsub-connector ./target/release/sumsub-connector

ENTRYPOINT ["./target/release/sumsub-connector"]