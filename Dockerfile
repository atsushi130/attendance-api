FROM rustlang/rust:nightly

RUN curl -sL --proto-redir -all, https https://raw.githubusercontent.com/atsushi130/attendance-api/master/install.sh| sh
WORKDIR attendance-api

RUN apt-get update
RUN apt-get install -y sqlite3
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
EXPOSE 8000

CMD cargo run --release
