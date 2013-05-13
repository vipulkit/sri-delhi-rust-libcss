SRI_HOME = ../../..
LIBWAPCAPLET = $(SRI_HOME)/libwapcaplet

SRC = basictests.rs
EXEC = basictests

$(EXEC): $(SRC)
	rustc --test -L $(LIBWAPCAPLET) $<

run: $(SRC)
	./$(EXEC)
