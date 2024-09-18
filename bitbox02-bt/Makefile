TOOLS=arm-none-eabi-
SIZE=${TOOLS}size
OBJCOPY=${TOOLS}objcopy
TARGET=thumbv6m-none-eabi
CARGO_FLAGS=-Zbuild-std=core,alloc \
	    -Zbuild-std-features=panic_immediate_abort,optimize_for_size

# For debugging linking / compiler flag issues use CC_ENABLE_DEBUG_OUTPUT=1 and
# pass -vv to cargo

.PHONY: build
build:
	@cargo +nightly build ${CARGO_FLAGS} --target ${TARGET}
	@${SIZE} target/${TARGET}/debug/bitbox02-bt
	@${OBJCOPY} -Obinary target/${TARGET}/debug/bitbox02-bt target/${TARGET}/debug/bitbox02-bt.bin

.PHONY: build-release
build-release:
	@cargo +nightly build --release ${CARGO_FLAGS} --target ${TARGET}
	@${SIZE} target/${TARGET}/release/bitbox02-bt
	@${OBJCOPY} -Obinary target/${TARGET}/release/bitbox02-bt target/${TARGET}/release/bitbox02-bt.bin

.PHONY: run
run:
	@cargo +nightly run ${CARGO_FLAGS} --target ${TARGET}

.PHONY: run-release
run-release:
	@cargo +nightly run --release ${CARGO_FLAGS} --target ${TARGET}

.PHONY: clean
clean:
	@cargo clean

.PHONY: gdb-server
gdb-server:
	JLinkGDBServer -device da14531 -if SWD -ir
