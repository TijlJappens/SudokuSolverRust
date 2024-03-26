FROM ubuntu as base
ENV RUSTUP_INIT_SKIP_PATH_CHECK=true
RUN apt update -y && apt install -y curl build-essential \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Add Cargo's bin directory to PATH
ENV PATH="/root/.cargo/bin:${PATH}"
FROM base as development