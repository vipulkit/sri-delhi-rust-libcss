SRI_HOME = ../..
LIB_PAR = -L $(SRI_HOME)/libparserutils 
LIB_WAP = -L $(SRI_HOME)/libwapcaplet 
LIB_CSS = -L $(SRI_HOME)/libcss 

all: 

	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *.so
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) testutils.rs 
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) parse.rs 
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump.rs  -Z verbose -Z debug-info
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump2.rs  -Z verbose -Z debug-info
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump_computed.rs 
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . css21.rs  -Z verbose -Z debug-info
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse-auto.rs  -Z verbose -Z debug-info
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse2-auto.rs  -Z verbose -Z debug-info
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) lex.rs 
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . lex-auto.rs 
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . csdetect.rs 
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) number.rs 
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . select-auto.rs  -Z verbose -Z debug-info

debug: 

	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *.so
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) testutils.rs 
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) parse.rs 
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump2.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump_computed.rs 
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . css21.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse-auto.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse2-auto.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) lex.rs 
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . lex-auto.rs 
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . csdetect.rs 
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) number.rs 
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . select-auto.rs  -Z verbose -Z debug-info

run: 
	./lex-auto
	./css21
	./parse-auto
	./parse2-auto
	./select-auto

clean:
	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *select-auto *number *lex-auto *.so
