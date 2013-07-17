SRI_HOME = ../..
LIB_PAR = -L $(SRI_HOME)/libparserutils 
LIB_WAP = -L $(SRI_HOME)/libwapcaplet 
LIB_CSS = -L $(SRI_HOME)/libcss 

all: 

	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *select-auto  *.so
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) testutils.rs -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) parse.rs -O
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump.rs  -O
	rustc  $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump_computed.rs 
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . css21.rs  -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse-auto.rs  -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse2-auto.rs  -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) lex.rs -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . lex-auto.rs -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . csdetect.rs -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) number.rs -O
	rustc  --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . select-auto.rs  -O
	cp -f select-auto ../utils/

debug: 

	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *select-auto *.so
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) testutils.rs -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) parse.rs -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump2.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) dump_computed.rs 
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . css21.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse-auto.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . parse2-auto.rs  -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) lex.rs -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . lex-auto.rs -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . csdetect.rs -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) number.rs -Z verbose -Z debug-info
	rustc  -Z verbose -Z debug-info --test $(LIB_PAR) $(LIB_CSS) $(LIB_WAP) -L . select-auto.rs  -Z verbose -Z debug-info
	cp -f select-auto ../utils/

run: 
	./css21
	./csdetect
	./parse
	./parse-auto
	./parse2-auto
	./lex
	./select-auto
	./lex-auto

clean:
	rm -f *css21 *csdetect *parse *parse-auto *parse2-auto *lex *select-auto *number *lex-auto *.so

copy-demo:
	cp -f select-auto ../utils/
