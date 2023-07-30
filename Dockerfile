FROM rust:bullseye
RUN \
  apt-get update && \
  apt-get install git -y && \
  mkdir ~/.ssh/ && \
  ssh-keyscan -t rsa github.com > ~/.ssh/known_hosts
WORKDIR /air-quality
COPY src .
RUN ls /air-quality
RUN cd /air-quality
RUN cargo build
CMD ["target/debug/air-quality"]
EXPOSE 8030
