FROM rust

WORKDIR /usr/library-of-despair/
COPY . .

RUN ["cargo", "build", "--release"]

ENTRYPOINT [ "cargo", "run", "--release" ]
