SRI_HOME = ../../..
LIB = -L $(SRI_HOME)/libparserutils -L $(SRI_HOME)/libwapcaplet -L $(SRI_HOME)/libcss -L .

SRC = parse.rs lex.rs csdetect.rs number.rs css21.rs lex-auto.rs
EXEC = $(SRC:%.rs=%)
UTIL_SRC = testutils.rs
UTIL = $(UTIL_SRC:%.rs=%.so)

.DEFAULT_GOAL := all

all: $(EXEC) $(SRC) $(UTIL_SRC)

%.so: %.rs
	rustc $(LIB) $<

% : %.rs $(UTIL)
	rustc --test $(LIB) $<

run : $(EXEC)
	./csdetect
	./number
	./parse
	./lex
	./lex-auto


.phony: clean

clean:
	rm  -f $(EXEC) *.so
