FROM rust:alpine3.18

EXPOSE 34254/udp

WORKDIR /home

COPY . .

RUN cargo build

CMD ["cargo", "run"]

