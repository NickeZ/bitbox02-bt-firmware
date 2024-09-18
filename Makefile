.PHONY: build
build:
	${MAKE} -C bitbox02-bt $@

.PHONY: build-release
build-release:
	${MAKE} -C bitbox02-bt $@

.PHONY: run
run:
	${MAKE} -C bitbox02-bt $@

.PHONY: run-release
run-release:
	${MAKE} -C bitbox02-bt $@

.PHONY: clean
clean:
	${MAKE} -C bitbox02-bt $@

.PHONY: print-linker-variables
print-linker-variables:
	@cargo run -p da14531-sdk-linker --release
