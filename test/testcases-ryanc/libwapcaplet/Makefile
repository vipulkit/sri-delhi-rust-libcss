SRI_HOME = ../../..
LIBWAPCAPLET = $(SRI_HOME)/libwapcaplet

PROJ = basictest
SRC = basictests.rs

$(PROJ): $(SRC)
	rustc --test -L $(LIBWAPCAPLET) $(SRC)

run: $(PROJ)
	./basictests
