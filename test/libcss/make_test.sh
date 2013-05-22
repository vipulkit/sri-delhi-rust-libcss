# do clean
rm *css21 *csdetect *parse *parse-auto *parse2-auto *lex *number *lex-auto 
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet css21.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse-auto.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse2-auto.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet lex.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet lex-auto.rs -L .
rustc --test -L ../../libparserutils/ csdetect.rs -L .
rustc --test -L ../../libparserutils -L ../../libwapcaplet/ -L ../../libcss number.rs -L .