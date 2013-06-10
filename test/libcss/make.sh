#!/bin/bash

# do clean

rm *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *.so
rustc -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet testutils.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet css21.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse.rs
rustc -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet dump.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet -L . parse-auto.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse2-auto.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet lex.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet -L . lex-auto.rs
rustc --test -L ../../libparserutils/  -L ../../libcss -L ../../libwapcaplet -L . csdetect.rs
rustc --test -L ../../libparserutils -L ../../libwapcaplet/ -L ../../libcss number.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet select-auto.rs
