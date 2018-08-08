FROM rust
EXPOSE 12306

RUN mkdir -p /data/logs
RUN touch /data/logs/server.log
RUN mkdir -p /data/rust_socket
COPY . /data/rust_socket/
# COPY . .
# VOLUME /data/rust_socket
WORKDIR /data/rust_socket
#RUN tail -f /dev/null

RUN cargo install

#RUN cat /data/rust_socket/test.file
