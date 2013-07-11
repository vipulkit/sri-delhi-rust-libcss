SRI_HOME = ../../..
LIB = -L $(SRI_HOME)/libparserutils -L $(SRI_HOME)/libwapcaplet -L $(SRI_HOME)/libcss -L $(SRI_HOME)/test/libcss -L .

# SRC = parse.rs lex.rs csdetect.rs number.rs css21.rs lex-auto.rs
SRC = select-auto.rs parse2-auto.rs parse-auto.rs csdetect.rs css21.rs lex.rs lex-auto.rs number.rs
EXEC = $(SRC:%.rs=%)
UTIL_SRC = testutils.rs
UTIL = $(UTIL_SRC:%.rs=%.so)

.DEFAULT_GOAL := all

all: $(EXEC) $(SRC) $(UTIL_SRC) test

.phony: run clean

# %.so: %.rs
	# rustc $(LIB) $<

# % : %.rs $(UTIL)
	# rustc --test $(LIB) $<
	# rustc $(LIB) $<

test: test.rs
	rustc  $(LIB) $<

testutils: testutils.rs
	rustc $(LIB) $<

select-auto: select-auto.rs
	rustc $(LIB) $<

parse2-auto: parse2-auto.rs
	rustc $(LIB) $<

parse-auto: parse-auto.rs
	rustc $(LIB) $<

lex-auto: lex-auto.rs
	rustc $(LIB) $<

csdetect: csdetect.rs
	rustc --test $(LIB) $<

css21: css21.rs
	rustc --test $(LIB) $<

lex: lex.rs
	rustc --test $(LIB) $<

number: number.rs
	rustc --test $(LIB) $<

run : $(EXEC)
	./csdetect
	./number
	./lex
	./lex-auto
	./parse

clean:
	rm  -f $(EXEC) *.so
