SRI_HOME = ../..
LIB = -L $(SRI_HOME)/libparserutils 
EXEC = $(SRC:%.rs=%)

all: $(EXEC) 
	rustc --test $(LIB) filter.rs  --link-args -liconv_wrapper -O

debug: $(EXEC)
	rustc -Z verbose -Z debug-info --test $(LIB) filter.rs  --link-args -liconv_wrapper

run: $(EXEC)
	./$(EXEC)

clean:
	rm -f $(EXEC)
	rm -f *.so