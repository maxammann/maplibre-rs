#!/usr/bin/env just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

set shell := ["bash", "-c"]

export NIGHTLY_TOOLCHAIN := "nightly-2022-04-04-x86_64-unknown-linux-gnu"

test:
  cargo test

install-clippy:
  rustup component add clippy

check PROJECT ARCH: install-clippy
  cargo clippy --no-deps -p {{PROJECT}} --target {{ARCH}}

install-rustfmt:
  rustup component add rustfmt

fmt: install-rustfmt
  cargo fmt --all --

fmt-check: install-rustfmt
  cargo fmt --all -- --check

default-toolchain:
  # Setups the toolchain from rust-toolchain.toml
  cargo --version > /dev/null

nightly-toolchain:
  rustup install $NIGHTLY_TOOLCHAIN
  rustup component add rust-src --toolchain $NIGHTLY_TOOLCHAIN

nightly-toolchain-android: nightly-toolchain
  rustup target add --toolchain $NIGHTLY_TOOLCHAIN x86_64-linux-android
  rustup target add --toolchain $NIGHTLY_TOOLCHAIN aarch64-linux-android

web-install PROJECT:
  cd web/{{PROJECT}} && npm install

web-lib TARGET: nightly-toolchain (web-install "lib")
  export RUSTUP_TOOLCHAIN=$NIGHTLY_TOOLCHAIN && cd web/lib && npm run {{TARGET}}

web-demo TARGET: (web-install "demo")
  cd web/demo && npm run {{TARGET}}

#profile-bench:
# cargo flamegraph --bench render -- --bench

build-android: print-android-env
  export RUSTUP_TOOLCHAIN=$NIGHTLY_TOOLCHAIN && cd android/gradle && ./gradlew assembleDebug

# language=bash
print-android-env:
  #!/usr/bin/env bash
  set -euxo pipefail
  echo "ANDROID_HOME: $ANDROID_HOME"
  echo "ANDROID_SDK_ROOT: $ANDROID_SDK_ROOT"
  echo "ANDROID_NDK_ROOT: $ANDROID_NDK_ROOT"

INNER_FRAMEWORK_PATH := "Products/Library/Frameworks/maplibre_rs.framework"
XC_FRAMEWORK_DIRECTORY := "./apple/MapLibreRs/"
export XC_FRAMEWORK_PATH := "./apple/MapLibreRs/MapLibreRs.xcframework"
PROJECT_DIR := "./apple/xcode/maplibre-rs.xcodeproj"
BINARY_NAME := "maplibre_rs"
BUILD_DIR := "./apple/build"

xcodebuild-archive ARCH PLATFORM:
  xcodebuild ARCHS="{{ARCH}}" archive -project "{{PROJECT_DIR}}" \
                                    -scheme "maplibre-rs" \
                                    -destination "generic/platform={{PLATFORM}}" \
                                    -archivePath "{{BUILD_DIR}}/{{ARCH}}-apple-{{PLATFORM}}"

# language=bash
xcodebuild-archive-fat EXISTING_ARCH EXISTING_PLATFORM ARCH: (xcodebuild-archive ARCH EXISTING_PLATFORM)
  #!/usr/bin/env bash
  set -euxo pipefail
  archive="{{BUILD_DIR}}/{{ARCH}}-apple-{{EXISTING_PLATFORM}}.xcarchive"
  existing_archive="{{BUILD_DIR}}/{{EXISTING_ARCH}}-apple-{{EXISTING_PLATFORM}}.xcarchive"
  fat_archive="{{BUILD_DIR}}/{{EXISTING_ARCH}}-{{ARCH}}-apple-{{EXISTING_PLATFORM}}.xcarchive"
  cp -R "$existing_archive" "$fat_archive"
  inner="$archive/{{INNER_FRAMEWORK_PATH}}"
  existing_inner="$existing_archive/{{INNER_FRAMEWORK_PATH}}"
  fat_inner="$fat_archive/{{INNER_FRAMEWORK_PATH}}"
  
  target_binary="$fat_inner/$(readlink -n "$fat_inner/{{BINARY_NAME}}")"
  lipo -create  "$existing_inner/{{BINARY_NAME}}" \
                "$inner/{{BINARY_NAME}}" \
                -output "$target_binary"
  cp -R $inner/Modules/{{BINARY_NAME}}.swiftmodule/* \
        "$fat_inner/Modules/{{BINARY_NAME}}.swiftmodule/"
  

xcodebuild-clean:
  rm -rf {{BUILD_DIR}}/*.xcarchive
  rm -rf {{XC_FRAMEWORK_DIRECTORY}}/*.xcframework

# language=bash
xcodebuild-xcframework: xcodebuild-clean (xcodebuild-archive  "arm64" "iOS") (xcodebuild-archive  "arm64" "macOS") (xcodebuild-archive  "arm64" "iOS Simulator") (xcodebuild-archive-fat "arm64" "macOS" "x86_64")
  #!/usr/bin/env bash
  set -euxo pipefail
  tuples=(
    "arm64,iOS"
    "arm64,iOS Simulator"
    "arm64-x86_64,macOS"
  )
  framework_args=$(for i in "${tuples[@]}"; do IFS=","; set -- $i; echo -n "-framework \"{{BUILD_DIR}}/$1-apple-$2.xcarchive/{{INNER_FRAMEWORK_PATH}}\" "; done)
  echo "$framework_args"
  echo  "$XC_FRAMEWORK_PATH"
  echo "$framework_args" | xargs xcodebuild -create-xcframework -output "$XC_FRAMEWORK_PATH"
  cat "$XC_FRAMEWORK_PATH/Info.plist"

book-serve:
  mdbook serve docs

# language=bash
extract-tiles:
  #!/usr/bin/env bash
  set -euxo pipefail
  if ! command -v tilelive-copy &> /dev/null
  then
    echo "tilelive-copy could not be found. Install it with 'yarn global add @mapbox/tilelive @mapbox/mbtiles'"
    exit 1
  fi
  # Bounds copied from https://boundingbox.klokantech.com/
  tilelive-copy \
    --minzoom=12 --maxzoom=12 \
    --bounds="11.395769,48.083436,11.618242,48.220866" \
    test-data/europe_germany-2020-02-13-openmaptiles-v3.12.1.mbtiles test-data/munich-12.mbtiles
  tilelive-copy \
    --minzoom=15 --maxzoom=15 \
    --bounds="11.395769,48.083436,11.618242,48.220866" \
    test-data/europe_germany-2020-02-13-openmaptiles-v3.12.1.mbtiles test-data/munich-15.mbtiles
