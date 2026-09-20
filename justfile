setup:
    sudo apt update
    sudo apt install -y build-essential clang mold sccache pkg-config \
        libwayland-dev libxkbcommon-dev libx11-dev libxi-dev \
        libxcursor-dev libxrandr-dev libasound2-dev libudev-dev

run:
    cargo run -F dev

build:
    cargo build --release --no-default-features
