all:
	cargo build --release --target=arm-linux-androideabi
	adb push target/arm-linux-androideabi/release/whatislife_enum /data/local/tmp/whatislife_enum
	adb shell chmod 755 /data/local/tmp/whatislife_enum
	adb shell touch /data/local/tmp/results.txt
	adb shell /data/local/tmp/whatislife_enum create > results.txt
	adb push results.txt /data/local/tmp

