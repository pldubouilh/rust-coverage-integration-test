COVERAGE_TARGET := ./target_coverage
COVERAGE_MYCODE_BIN := ./target_coverage/debug/mycode

test::
# normal test, using regular mycode binary
	cargo build -p mycode
	cargo test -p integration-test

check::
	@which grcov || (echo "grcov needed"; exit 1)

coverage:: check
# build mycode with instrumentation coverage support
# executing this binary will generate a traces/*.profraw file at $PWD that can be analysed later on
# uses special target dir so that coverage builds are cached (rustflag invalidates cache aggressively)
	rm -rf traces
	mkdir -p traces
	CARGO_TARGET_DIR=${COVERAGE_TARGET} RUSTFLAGS="-C instrument-coverage" cargo build -p mycode
	COVERAGE_BIN=${COVERAGE_MYCODE_BIN} cargo test -p integration-test

coverage-report:: coverage
	rm -rf out-cov
	grcov -s ../ -b ${COVERAGE_MYCODE_BIN} -t html -o out-cov traces/*.profraw
	@-sleep 2 && open http://127.0.0.1:8000 || xdg-open http://127.0.0.1:8000 &
	python -m http.server --directory out-cov

coverage-summary:: coverage
	llvm-profdata merge -sparse traces/*.profraw -o out.profdata
	llvm-cov report -instr-profile=out.profdata --summary-only ${COVERAGE_MYCODE_BIN} ../mycode

coverage-cli-report:: coverage
	llvm-profdata merge -sparse traces/*.profraw -o out.profdata
	llvm-cov show -Xdemangler=rustfilt ${COVERAGE_MYCODE_BIN} \
		-instr-profile=out.profdata \
		-show-line-counts-or-regions \
		-show-instantiations \
		-name=add_quoted_string \
		../mycode