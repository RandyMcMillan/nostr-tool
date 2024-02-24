FROM rust:latest as base
LABEL org.opencontainers.image.source="https://github.com/gnostr-org/gnostr"
LABEL org.opencontainers.image.description="gnostr-docker"
RUN touch updated
RUN echo $(date +%s) > updated
RUN apt-get update
RUN apt-get install automake bash cmake jq make libssl-dev lsof pkg-config protobuf-compiler python-is-python3 systemd tcl-dev -y
RUN chmod +x /usr/bin/systemctl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /tmp
RUN git clone --recurse-submodules -j4 --branch 1708649039/3b67df8f5/dae700e50  --depth 10 https://github.com/gnostr-org/gnostr.git
WORKDIR /tmp/gnostr
RUN make detect
RUN for t in */Cargo.toml;do echo ;cargo b -r -vv --manifest-path  ;done
RUN make gnostr-am
RUN cmake .
RUN make gnostr all
RUN make gnostr-install
RUN make        install
RUN cargo install --path bins --force
RUN install ./serve /usr/local/bin || true
ENV PATH=$PATH:/usr/bin/systemctl
RUN ps -p 1 -o comm=
EXPOSE 80 6102 8080 ${PORT}
VOLUME /src
FROM base as gnostr

