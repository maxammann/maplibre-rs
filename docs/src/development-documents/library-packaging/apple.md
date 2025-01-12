# Apple

On Apple maplibre-rs is packaged as:
* Multiple .xcarchive packages which include a framework. Each for a different architecture and platform.
* A single .xcframework package which contains multiple frameworks of different architectures and platforms.
* A swift package which just references the .xcframework package and makes distributing easier.

The following diffs are extracted from [this diff](../../../../apple/framework.diff). They should serve as documentation
for the XCode project. This is required because XCode is a mess.

## XCode Project description

### Library Entry

{{#include figures/diff-maplibre-swift.html}}

The swift code above is the main entry for the Swift API. From this entry file we can expose more API of maplibre-rs.
Any C functions which are referenced in the XCode framework's header are available automatically in Swift.

### Framework

{{#include figures/diff-xcode-project-framework.html}}

The framework needs to link against the static library `libmaplibre_apple.a`, which has been generated by Cargo.
In order to allow XCode to dynamically select the library based on the `Library Search Path` (Build Settings) one needs
to add a relative file to XCode. The entry in the `project.pbxproj` should look like that:

```js
B085D5A32812987B00906D21 /* libmaplibre_apple.a */ = {
    isa = PBXFileReference;
    lastKnownFileType = archive.ar;
    path = libmaplibre_apple.a;
    sourceTree = SOURCE_ROOT;
};
```

Note the `path = libmaplibre_apple.a`. This path does not link to a concrete file, but to a file which can be found
during building.

A file can be added to the frameworks and library link phase in XCode.

### Cargo Build Phase

{{#include figures/diff-xcode-project-build-cargo.html}}

In order to trigger Cargo builds when starting a XCode build we include a `Cargo Build` script. This build script needs
to run before the linking phase (drag and drop it to the top).

The following build script builds based on XCode environment variables the correct static library. We depend on
the `$ARCHS`
environment variable, as the others seem unreliable. Note that this can include multiple architectures, unless the build
setting `ONLY_ACTIVE_ARCH` is set to `YES`.

```bash
. "$HOME/.cargo/env"

arch="unknown"
vendor="apple"
os_type="unknown"
environment_type=""

mode=""

echo "ARCH: $ARCHS"

if [[ $CONFIGURATION == "Release" ]]
then
    mode="--release"
fi

if [[ $ARCHS == "x86_64" ]]
then
    arch="x86_64"
elif [[ $ARCHS == "arm64" ]]
then
    arch="aarch64"
fi

if [[ $SDK_NAME == *"iphoneos"* ]]
then
    os_type="ios"
elif [[ $SDK_NAME == *"macos"* ]]
then
    os_type="darwin"
elif [[ $SDK_NAME == *"iphonesimulator"* ]]
then
    os_type="ios"
    environment_type="sim"
fi


triplet="$arch-$vendor-$os_type"

if [ -n "$environment_type" ]
then
    triplet="$triplet-$environment_type"
fi

echo "$mode"
echo "$triplet"

env -i zsh -c "cargo build -p maplibre-apple $mode --target $triplet --lib"
```

### Build Settings

{{#include figures/diff-xcode-project-build-settings.html}}

Explanations for the settings:

* `BUILD_LIBRARY_FOR_DISTRIBUTION`: Define that this is a library (effect unknown to me)
* `CODE_SIGN_STYLE`: The framework is not signed
* `DEVELOPMENT_TEAM`: No development team is set
* `LIBRARY_SEARCH_PATHS[sdk=x][arch=y]`: We set the path for the `libmaplibre_apple.a` lies
* `MACH_O_TYPE` / `SKIP_INSTALL`: If this is not set to `staticlib` and `NO`, then the `libmaplibre_apple.a` binary is not included in the final framework xcarchive.
* `SUPPORTED_PLATFORMS`: Explicitly says that this library works on any platform. 
* `SUPPORTS_MACCATALYST`: Explicitly says that this library works on Mac Catalyst.

The same settings are done for Release and Debug.

## xcframework packaging

Creating a xcframework is usually quite straight forward. Just execute the following:

```bash
xargs xcodebuild -create-xcframework -framework ./a -framework ./b -output out.xcframework
```

Unfortunately, it is not possible to bundle some frameworks together like:

* macOS-arm64 and macOS-x86_64

In order to package these architectures and platforms together a fat binary needs to be created using the `lipo` tool.
This means from two frameworks we create a unified framework with a fat binary.
There are two important steps:

1. Create a fat binary using `lipo -create binA binB -output binfat`
2. Copy for example the arm64 framework and add the `.swiftmodule` definitions from the x86_64 framework

## Single UIApplication

Right now `winit` only allows the usage of a `UIApplication`. This means the application needs to run in fullscreen.
[Tracking Issue](https://github.com/maplibre/maplibre-rs/issues/28)

## Example App

The following settings are important for the example application within the XCode project.

### Info Plist for Applications

{{#include figures/diff-xcode-project-info-plist.html}}

* The `INFOPLIST_KEY_UIApplicationSceneManifest_Generation` needs to be unset. Else the application screen is just black.

### Files & Assets

{{#include figures/diff-xcode-project-assets.html}}

* The example/demo application within the XCode project references the `maplibre_rs.framework`. Some default files have
  been removed.

### MacOS Entitlements

{{#include figures/diff-macOS-entitlements.html}}

* On macOS one needs to allow network access via `com.apple.security.network.client`