#! /bin/sh
#rm -rf *.so *.o 
rustc -L . aliases.rs
rustc -L . filter-segv.rs
