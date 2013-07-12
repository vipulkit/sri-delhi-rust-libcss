SRI_HOME = ../..
LIB_WAP = -L $(SRI_HOME)/libwapcaplet 

all: 

	rustc  $(LIB_WAP) perf-test.rs --opt-level=2
	rustc  $(LIB_WAP) wapcaplet2_perf_test.rs --opt-level=2
	rustc  $(LIB_WAP) wapcaplet2_perf_test_thread_safe.rs --opt-level=2
	rustc  $(LIB_WAP) wapcaplet2_basictests.rs --opt-level=2

run: 

clean:
	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *.so
