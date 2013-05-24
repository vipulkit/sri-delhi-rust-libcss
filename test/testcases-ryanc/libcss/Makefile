SRI_HOME = ../../..
LIB = -L $(SRI_HOME)/libparserutils -L $(SRI_HOME)/libwapcaplet -L $(SRI_HOME)/libcss

SRC = parse.rs lex.rs csdetect.rs
EXEC = $(SRC:%.rs=%)

all: $(EXEC) $(SRC)

% : %.rs
	rustc --test $(LIB) $<

run : $(EXEC)
	./parse

.phony: clean

clean:
	rm  -f $(EXEC)
