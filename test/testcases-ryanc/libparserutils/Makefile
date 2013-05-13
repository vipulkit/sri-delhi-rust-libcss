SRI_HOME = ../../..
LIB = $(SRI_HOME)/libparserutils

SRC = aliases.rs inputstream.rs filter.rs
EXEC = $(SRC:%.rs=%)

all: $(EXEC) $(SRC)

% : %.rs
	rustc --test -L $(LIB) $<

test : $(EXEC)
	./aliases
	./inputstream

.phony: clean

clean:
	rm  -f $(EXEC)
