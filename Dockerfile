FROM rustlang/rust:nightly

RUN curl -sL --proto-redir -all, https https://raw.githubusercontent.com/atsushi130/attendance-api/master/install.sh| sh
WORKDIR attendance-api

CMD cargo run --release
