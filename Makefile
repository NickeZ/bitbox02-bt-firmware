TOOLCHAIN_PREFIX=arm-none-eabi-
TARGET=thumbv6m-none-eabi
CARGO_FLAGS=-Zbuild-std=core,alloc \
	    -Zbuild-std-features=panic_immediate_abort,optimize_for_size

.PHONY: build
build:
	@cargo +nightly build -p bitbox02-bt ${CARGO_FLAGS} --target ${TARGET}
	@${TOOLCHAIN_PREFIX}size target/${TARGET}/debug/bitbox02-bt

.PHONY: build-release
build-release:
	@cargo +nightly build -p bitbox02-bt --release ${CARGO_FLAGS} --target ${TARGET}
	@${TOOLCHAIN_PREFIX}size target/${TARGET}/release/bitbox02-bt

.PHONY: run
run:
	@cargo +nightly run -p bitbox02-bt ${CARGO_FLAGS} --target ${TARGET}

.PHONY: run-release
run-release:
	@cargo +nightly run -p bitbox02-bt --release ${CARGO_FLAGS} --target ${TARGET}

.PHONY: clean
clean:
	@cargo clean

.PHONY: print-linker-variables
print-linker-variables:
	@cargo run -p da14531-sdk-linker --release
