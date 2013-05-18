# do clean
rm -rf *.so *.o parse/parse parse/parse-auto parse/parse2-auto
rustc --test -L ../../libparserutils -L ../../libcss  -L ../../libwapcaplet css21.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss   -L ../../libwapcaplet parse/parse.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse/parse-auto.rs -L .
rustc --test -L ../../libparserutils -L ../../libcss -L ../../libwapcaplet parse/parse2-auto.rs -L .