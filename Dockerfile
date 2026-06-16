FROM rust

WORKDIR /usr/library-of-despair/
COPY . .
ARG SQLX_OFFLINE=true
RUN ["cargo", "build", "--release"]

ENTRYPOINT [ "cargo", "run", "--release" ]
