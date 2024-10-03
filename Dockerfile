# Copyright 2024 Shift Crypto AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM ubuntu:22.04

# arm-none-eabi-gdb depends on libncurses5 and libtinfo5
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y \
    curl \
    make \
    gcc \
    xz-utils \
    bzip2 \
    unzip \
    patch \
    gnupg2 \
    libncurses5 \
    libtinfo5 \
    && rm -rf /var/lib/apt/lists/*

# for llvm-18, see https://apt.llvm.org/
RUN echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" >> /etc/apt/sources.list && \
    echo "deb-src http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" >> /etc/apt/sources.list && \
    curl -L https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y \
    clang-18 \
    && rm -rf /var/lib/apt/lists/*

# Install ARM GNU Toolchain
ARG TARGETPLATFORM
RUN if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
      GNU_TOOLCHAIN=https://developer.arm.com/-/media/Files/downloads/gnu/13.3.rel1/binrel/arm-gnu-toolchain-13.3.rel1-aarch64-arm-none-eabi.tar.xz \
      GNU_TOOLCHAIN_HASH=c8824bffd057afce2259f7618254e840715f33523a3d4e4294f471208f976764 \
      GNU_TOOLCHAIN_FORMAT=xz; \
    else \
      GNU_TOOLCHAIN=https://developer.arm.com/-/media/Files/downloads/gnu-rm/8-2018q4/gcc-arm-none-eabi-8-2018-q4-major-linux.tar.bz2 \
      GNU_TOOLCHAIN_HASH=fb31fbdfe08406ece43eef5df623c0b2deb8b53e405e2c878300f7a1f303ee52 \
      GNU_TOOLCHAIN_FORMAT=bz2; \
    fi; \
    curl -sSLo gcc.tar.${GNU_TOOLCHAIN_FORMAT} ${GNU_TOOLCHAIN} && \
    echo "$GNU_TOOLCHAIN_HASH gcc.tar.${GNU_TOOLCHAIN_FORMAT}" | sha256sum -c && \
    tar -xf gcc.tar.${GNU_TOOLCHAIN_FORMAT} -C /usr/local --strip-components=1 && \
    rm -f gcc.tar.${GNU_TOOLCHAIN_FORMAT}

# The da14531 SDK is not publicly available. So there needds to be a copy of it
# in the same folder where the Dockerfile is.
ENV SDK_PATH=/opt/DA145xx_SDK/6.0.22.1401
RUN --mount=source=.,target=/mnt \
    cd /opt && \
    unzip /mnt/SDK_6.0.22.1401.zip && \
    for patch in `ls /mnt/da14531-sdk/patches`; do patch -p1 < /mnt/da14531-sdk/patches/$patch; done

# Install rust compiler
ENV PATH=/opt/cargo/bin:$PATH RUSTUP_HOME=/opt/rustup
RUN --mount=source=rust-toolchain.toml,target=/mnt/rust-toolchain.toml \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | CARGO_HOME=/opt/cargo sh -s -- --default-toolchain $(grep -oP '(?<=channel = ")[^"]+' /mnt/rust-toolchain.toml) -y && \
    rustup target add thumbv6m-none-eabi && \
    rustup component add \
    rustfmt \
    clippy \
    rust-src
