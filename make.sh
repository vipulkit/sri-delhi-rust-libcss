#! /bin/sh
rm -rf *.so *.o 
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
gcc -fPIC -shared iconv_wrapper.c -o libiconv_wrapper.so 
rustc -L . iconv_crate.rc 
rustc -L . parserutils.rs
rustc -L . parserutils_filter.rs
rustc -L . parserutils_inputstream.rs
rustc -L . wapcaplet.rs
rustc -L . css_enum.rs
rustc -L . css_ds.rs
rustc -L . css.rs

#cd test
#./make_test.sh
#cd ..
