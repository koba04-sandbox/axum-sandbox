FROM rust:1.74-alpine as builder
RUN apk add pkgconfig openssl-dev libc-dev
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk add openssl ca-certificates
COPY --from=builder /app/target/release/toy-app /
EXPOSE 3000
CMD ["./toy-app"]
