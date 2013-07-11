SRI_HOME = ../..
LIB = -L $(SRI_HOME)/libparserutils 
SRC = filter.rs
EXEC = $(SRC:%.rs=%)

all: $(EXEC) $(SRC)

% : %.rs
	cp ./../../libparserutils/*.so .
	rustc --test $(LIB) $<  --link-args -liconv_wrapper

debug: $(EXEC) $(SRC)

% : %.rs
	cp ./../../libparserutils/*.so .
	rustc -Z verbose -Z debug-info --test $(LIB) $<  --link-args -liconv_wrapper

run: $(EXEC)
	./$(EXEC)

clean:
	rm -f $(EXEC)
	rm -f *.so