FROM rust:latest

WORKDIR /usr/src/alerty
COPY . .

RUN cargo install --path .

WORKDIR /
RUN rm -rf /usr/src/alerty
RUN touch /alerty.toml

CMD ["alerty"]
