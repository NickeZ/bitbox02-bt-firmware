.PHONY: run
run:
	@cargo +nightly run -p bitbox02-bt \
		-Zbuild-std=core,alloc \
		-Zbuild-std-features=panic_immediate_abort,optimize_for_size \
		--release

.PHONY: clean
clean:
	@cargo clean
