OPTS="-Os"

factorial.xex: target/mos-unknown-none/debug/libatest.rlib main.c
	../../../llvm-mos/build/bin/clang target/mos-unknown-none/debug/deps/librustc_std_workspace_core-*.rlib target/mos-unknown-none/debug/deps/libcore-*.rlib main.c target/mos-unknown-none/debug/libatest.rlib -target mos-unknown-none --config ~/private/llvm-mos-sdk/build/atari/800xl.cfg ${OPTS} -o factorial.xex
factorial.s: target/mos-unknown-none/debug/libatest.rlib main.c
	../../../llvm-mos/build/bin/clang main.c target/mos-unknown-none/debug/libatest.rlib -target mos-unknown-none --config ~/private/llvm-mos-sdk/build/atari/800xl.cfg ${OPTS} -o factorial.s -Wl,--lto-emit-asm -flto=thin

target/mos-unknown-none/debug/libatest.rlib: src/lib.rs
	RUSTFLAGS="--emit=llvm-ir -C debuginfo=0" cargo +mos build -vv -j 1 $*

clean:
	cargo clean
	rm -f factorial.xex factorial.s*


