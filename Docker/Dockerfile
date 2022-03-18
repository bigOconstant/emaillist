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
RUN pwd
COPY migrations .
RUN ls;
WORKDIR /email/templates

COPY templates .

WORKDIR /email

RUN ls;


ENTRYPOINT ["/email/run.sh"]

CMD ["/email/emaillist"]
