File system enumerator and file monitor for Android. Built to be compatible with other command line utilties! This tool was created to somewhat automate file discovery by catching new files that are created by Android applications at runtime.

Disclaimer: This tool is meant for educational and ethical purposes. I'm not responsible for any buffoonery.
Tested on Ubuntu with rooted device.

---

Binaries are built in 

### Building from source with cargo

#### Pre-reqs

1. Install Rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Switch to Rust nightly for .contains substring library.

`rustup show`

`rustup override set <toolchain>`

3. Install toolchains for your Android architecture

Note: `adb shell uname -a` will list your phone's architecture.

`rustup target add arm-linux-androideabi --toolchain nightly`

Other toolchain names:

```
aarch64-linux-android
arm-linux-androideabi
armv7-linux-androideabi
i686-linux-android
x86_64-linux-android
```
4. Change .cargo/config to link your NDK SDK

```
[target.arm-linux-androideabi]
ar = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"
linker = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi24-clang"

[target.aarch64-linux-android]
ar = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
linker = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android24-clang"

[target.i686-linux-android]
ar = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"
linker = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android24-clang"

[target.x86_64-linux-android]
ar = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar"
linker = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android24-clang"

[target.armv7-linux-androideabi]
ar = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"
linker = "/home/b3nac/Android/Sdk/ndk/21.1.6352462/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi24-clang"
```

5. Build binaries

Creates the binary for your PC

`cargo build`

Creates a binary for arm-linux-androideabi

`cargo build --release --target=arm-linux-androideabi`

Creates a binary for aarch64-linux-android

`cargo build --release --target=aarch64-linux-android`

Creates a binary for armv7-linux-androideabi

`cargo build --release --target=armv7-linux-androideabi`

---

### Automatically deploy to a device with make files

#### Pre-reqs

`sudo apt-get install build-essential`

Note: This will install other build tools as well.

These make files combine several commands and automate the process.

```
all:
	cargo build --release --target=arm-linux-androideabi
	adb push target/arm-linux-androideabi/release/whatislife_enum /data/local/tmp/whatislife_enum
	adb shell chmod 755 /data/local/tmp/whatislife_enum
	adb shell /data/local/tmp/whatislife_enum create > results.txt
	adb push results.txt /data/local/tmp

```

##### ARM architecture

`make -f test_arm.mk`

##### ARMv7 architecture

`make -f test_armv7.mk`

##### AARCH64 architecture

`make -f test_aarch64.mk`

---

### Command documentation

For most of the commands to work the filesystem needs to be enumerated first.

`adb shell /data/local/tmp/whatislife_enum create > results.txt` will pipe the Android filesystem to your PC. After that either `adb push results.txt /data/local/tmp` or run whatislife_enum create in the tmp directory and pipe to a file on the device itself. `./whatislife_enum create > results.txt`. Then all the commands from your PC will work and output results.

##### Enumerate the entire file system

`adb shell /data/local/tmp/whatislife_enum create`

Save results by piping to a file `adb shell /data/local/tmp/whatislife_enum create > results.txt`

Note: > overwrites the entire file >> appends results

##### Enumerate apps

`adb shell /data/local/tmp/whatislife_enum apps path/to/result/file.txt`

Save results by piping to a file `adb shell /data/local/tmp/whatislife_enum apps /data/local/tmp/results.txt > filename.txt`

Note: > overwrites the entire file >> appends results

##### Enumerate external-storage

`adb shell /data/local/tmp/whatislife_enum external-storage path/to/result/file.txt`

Save results by piping to a file `adb shell /data/local/tmp/whatislife_enum external-storage /data/local/tmp/results.txt > filename.txt`

Note: > overwrites the entire file >> appends results

##### Show changes to file system

Scan the filesystem again and name the file whatever you want in this example I named the new scan results2.txt (super original I know).

`adb shell /data/local/tmp/whatislife_enum filemon /data/local/tmp/results.txt /data/local/tmp/results2.txt | sed '/proc/d'`

The result will print the differences between file system scans.

##### Update base file system file

`adb shell /data/local/tmp/whatislife_enum refresh current_result_file new_result_file_name new_result_file`

`adb shell /data/local/tmp/whatislife_enum refresh /data/local/tmp/results.txt /data/local/tmp/results-test.txt /data/local/tmp/results2.txt`
