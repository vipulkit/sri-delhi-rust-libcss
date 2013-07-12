SRI_HOME = ../../..
LIB = -L $(SRI_HOME)/libparserutils -L $(SRI_HOME)/libwapcaplet -L $(SRI_HOME)/libcss --link-args -liconv_wrapper -L./../../libparserutils/

SRC = aliases.rs inputstream.rs filter.rs
EXEC = $(SRC:%.rs=%)

all: $(EXEC) $(SRC)

% : %.rs
	rustc --test $(LIB) $<

run : $(EXEC)
	./aliases
	./inputstream
	./filter

.phony: clean

clean:
	rm  -f $(EXEC)
