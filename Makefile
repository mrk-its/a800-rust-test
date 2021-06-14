ELF_RUST_FLAGS="-Cdebuginfo=0 -Clinker=../llvm-mos/build/bin/clang -Clink-arg=--config -Clink-arg=cfg/800xl_elf.cfg -Cdefault-linker-libraries=y"

all:
	cargo +mos build -vv

target/mos-unknown-none/debug/elf/a800_rust_test.xex:
	RUSTFLAGS=${ELF_RUST_FLAGS} cargo +mos build -vv -Z unstable-options --out-dir target/mos-unknown-none/debug/elf/
	python tools/elf2xex.py target/mos-unknown-none/debug/elf/a800_rust_test

elf: target/mos-unknown-none/debug/elf/a800_rust_test.xex

disasm:	elf
	../llvm-mos/build/bin/llvm-objdump -d --print-imm-hex target/mos-unknown-none/debug/elf/a800_rust_test

clean:
	cargo +mos clean
