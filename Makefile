gen:
	env UPDATE_EXPECT=1 cargo test --package cranelift-filetests --lib -- test_zkasm::tests::run_benchmarks --nocapture

run:
	./ci/test-zkasm.sh cranelift/zkasm_data/benchmarks/fibonacci/generated/from_rust.zkasm

bench:
	cargo run -p analyze-zkasm
