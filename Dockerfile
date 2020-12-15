FROM fedora:latest
WORKDIR /setup

# install tools
RUN dnf upgrade -y
RUN dnf -y install python java-latest-openjdk-devel fedora-packager rpmdevtools dpkg gcc-aarch64-linux-gnu gcc-arm-linux-gnu musl-gcc
RUN dnf -y groupinstall "Development Tools"

# download android ndk, build tools
RUN curl -o android-ndk.zip https://dl.google.com/android/repository/android-ndk-r21d-linux-x86_64.zip
RUN curl -o android-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-6858069_latest.zip
RUN unzip android-tools.zip
RUN rm android-tools.zip
RUN yes | cmdline-tools/bin/sdkmanager --sdk_root=/root/.android/sdk "build-tools;30.0.3" "platforms;android-30"
RUN curl -L -o /root/.bundletool-all.jar https://github.com/google/bundletool/releases/latest/download/bundletool-all-1.3.0.jar
RUN unzip android-ndk.zip
RUN rm android-ndk.zip

# install android ndk, sdk
RUN mkdir -p /root/.android/sdk/ndk
RUN mv /setup/android-ndk-r21d /root/.android/sdk/ndk/21.3.6528147
RUN /root/.android/sdk/ndk/21.3.6528147/build/tools/make_standalone_toolchain.py --api 30 --arch arm64 --install-dir ~/.android/ndk/arm64
RUN /root/.android/sdk/ndk/21.3.6528147/build/tools/make_standalone_toolchain.py --api 30 --arch arm --install-dir ~/.android/ndk/arm
ENV ANDROID_NDK_ROOT="/root/.android/sdk/ndk"
ENV ANDROID_SDK_ROOT="/root/.android/sdk"

# rustup, targets, cargo tools, minifier
RUN curl --proto '=https' --tlsv1.2 -sSf -o rustup.sh https://sh.rustup.rs
RUN chmod +x rustup.sh
RUN ./rustup.sh -y
ENV PATH="/root/.cargo/bin:/root/bin:${PATH}:/sbin"
RUN rustup target add x86_64-unknown-linux-musl aarch64-linux-android armv7-linux-androideabi wasm32-unknown-unknown
RUN cargo install cargo-license cargo-deb cargo-rpm
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN curl -L https://github.com/tdewolff/minify/releases/download/v2.8.0/minify_linux_amd64.tar.gz | tar -xz minify && mv minify /usr/local/bin/minify-v2.8.0

COPY . /app
WORKDIR /app
RUN make

CMD make
