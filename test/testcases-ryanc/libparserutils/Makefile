SRI_HOME = ../../..
LIB = -L $(SRI_HOME)/libparserutils -L $(SRI_HOME)/libwapcaplet -L $(SRI_HOME)/libcss

SRC = aliases.rs inputstream.rs filter.rs
EXEC = $(SRC:%.rs=%)

all: $(EXEC) $(SRC)

% : %.rs
	rustc --test $(LIB) $<

run : $(EXEC)
	./aliases
	./inputstream

.phony: clean

clean:
	rm  -f $(EXEC)
