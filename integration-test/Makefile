COVERAGE_TARGET := ./target_coverage
COVERAGE_BIN := ./target_coverage/debug/mycode

test::
# normal test, using regular mycode binary
	cargo build -p mycode
	cargo test -p integration-test

coverage::
# build mycode with instrumentation coverage support
# executing this binary will generate a traces/*.profraw file at $PWD that can be analysed later on
# uses special target dir so that coverage builds are cached (rustflag invalidates cache aggressively)
	rm -rf traces
	mkdir -p traces
	CARGO_TARGET_DIR=${COVERAGE_TARGET} RUSTFLAGS="-C instrument-coverage" cargo build -p mycode
	COVERAGE_BIN=${COVERAGE_BIN} cargo test -p integration-test

coverage-report:: coverage
	rm -rf out-cov
	grcov -s ../ -b ${COVERAGE_BIN} -t html -o out-cov traces/*.profraw
	@-sleep 2 && open http://127.0.0.1:8000 || xdg-open http://127.0.0.1:8000 &
	python -m http.server --directory out-cov

coverage-coveralls:: coverage
	rm -rf out-cov
	grcov -s ../ -b ${COVERAGE_BIN} -t lcov -o out-cov traces/*.profraw

merge::
	llvm-profdata merge -sparse traces/*.profraw -o out.profdata

coverage-summary:: coverage merge
	llvm-cov report -instr-profile=out.profdata --summary-only ${COVERAGE_BIN} ../mycode

coverage-percentage-gh:: coverage merge
	echo "COVERAGE_STAT=$(shell llvm-cov export -instr-profile=out.profdata -format=text --summary-only $(COVERAGE_BIN) ../mycode | jq .data[0].totals.lines.percent)" >> ${GITHUB_ENV}

coverage-cli-report:: coverage merge
	llvm-cov show -Xdemangler=rustfilt ${COVERAGE_BIN} \
		-instr-profile=out.profdata \
		-show-line-counts-or-regions \
		-show-instantiations \
		-name=add_quoted_string \
		../mycode

