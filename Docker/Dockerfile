FROM rust:1.59 as build
WORKDIR /email
COPY . .
RUN cargo build --release

FROM rust:1.59
WORKDIR /email
RUN cargo install diesel_cli --no-default-features --features sqlite
COPY --from=build /email/target/release/emaillist .
COPY ./Docker/run.sh .
WORKDIR /email/migrations
COPY migrations .
WORKDIR /email/templates
COPY templates .
WORKDIR /email
ENTRYPOINT ["/email/run.sh"]

CMD ["/email/emaillist"]
