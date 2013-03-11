#! /bin/sh
rm -rf *.so *.o testcpp testrs
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
gcc -fPIC -shared iconv_wrapper.c -o libiconv_wrapper.so 
rustc -L . iconv_crate.rc 
rustc -L . parserutils.rs
rustc -L . wapcaplet.rs

