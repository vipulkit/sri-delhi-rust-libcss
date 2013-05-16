SRI_HOME = ../../..
LIB = -L $(SRI_HOME)/libwapcaplet

SRC = basictests.rs
EXEC = $(SRC:%.rs=%)

all: $(EXEC) $(SRC)

% : %.rs
	rustc --test $(LIB) $<

run: $(EXEC)
	./$(EXEC)

clean:
	rm -f $(EXEC)
