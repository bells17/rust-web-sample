# syntax=docker/dockerfile:1
FROM --platform=$TARGETPLATFORM rust:buster AS chef
WORKDIR /workspace
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /workspace/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin rust-web-sample

FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12
ENV ROCKET_ENV=production
COPY --from=builder /workspace/target/release/rust-web-sample /rust-web-sample
CMD ["/rust-web-sample"]
