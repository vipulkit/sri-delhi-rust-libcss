#!/bin/bash

# do clean

rm *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto *.so
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet lex.rs
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet -L . lex-auto.rs
