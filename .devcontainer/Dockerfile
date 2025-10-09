FROM mcr.microsoft.com/devcontainers/rust:bullseye

# Install emscripten
RUN rustup target add wasm32-unknown-emscripten

RUN git clone https://github.com/emscripten-core/emsdk.git /usr/local/emsdk \
    && cd /usr/local/emsdk \
    && ./emsdk install latest \
    && ./emsdk activate latest

# We assume the username "vscode" here because the base image uses this
RUN echo "EMSDK_QUIET=1 source /usr/local/emsdk/emsdk_env.sh" >> /home/vscode/.bashrc
