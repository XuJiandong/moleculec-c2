OS = Unknown
ifneq ($(shell uname -a | grep -i Darwin),)
	OS = MacOS
endif
ifneq ($(shell uname -a | grep -i Linux),)
	OS = Linux
endif
ifeq ($(OS),Unknown)
	echo "error: unsupported OS"; exit 1
endif

NPROC?=4
CC=clang
LLVM_PROFDATA=llvm-profdata
LLVM_COV=llvm-cov

CORPUS_DIR=corpus

FUZZER_FLAGS=-g -O1 -fsanitize=fuzzer,address,undefined -I ../../include -I. -I ../blockchain

COVERAGE_DIR=coverage
COVERAGE_FLAGS=-fprofile-instr-generate -fcoverage-mapping -I ../../include -I . -I ../blockchain

ifeq ($(OS),MacOS)
	COVERAGE_FLAGS+=-Wl,-U,_LLVMFuzzerCustomMutator -Wl,-U,_LLVMFuzzerInitialize
endif

EXTERNAL_HEADERS=./fuzzer_func.h ../blockchain/blockchain-api2.h ../../include/molecule2_reader.h

all: fuzzer coverage

show: $(COVERAGE_DIR)/fuzzer.profdata
	$(LLVM_COV) show --instr-profile=$(COVERAGE_DIR)/fuzzer.profdata moleculec_c2_coverage

report: $(COVERAGE_DIR)/fuzzer.profdata coverage $(EXTERNAL_HEADERS)
	$(LLVM_COV) report --show-functions --instr-profile=$(COVERAGE_DIR)/fuzzer.profdata moleculec_c2_coverage $(EXTERNAL_HEADERS)

fuzzer: $(EXTERNAL_HEADERS)
	clang $(FUZZER_FLAGS) moleculec_c2_fuzzer.c -o moleculec_c2_fuzzer

coverage: $(EXTERNAL_HEADERS)
	clang $(COVERAGE_FLAGS) moleculec_c2_coverage.c moleculec_c2_fuzzer.c -o moleculec_c2_coverage

start-fuzzer: fuzzer
	./moleculec_c2_fuzzer -max_len=65536 -workers=$(NPROC) -jobs=$(NPROC) corpus

clean:
	rm -rf moleculec_c2_fuzzer moleculec_c2_coverage moleculec_c2_fuzzer.dSYM

#%.h:
#	ln -s $(CURDIR)/../$@ $(CURDIR)/$@

%.profraw: coverage
	LLVM_PROFILE_FILE=$@ ./moleculec_c2_coverage $(CORPUS_DIR)/*

%.profdata: %.profraw
	$(LLVM_PROFDATA) merge --sparse $< -o $@

.PHONY: all fuzzer coverage report

.PRECIOUS: $(COVERAGE_DIR)/fuzzer.profraw $(COVERAGE_DIR)/fuzzer.profdata
