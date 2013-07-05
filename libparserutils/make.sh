rm -rf *.so *.o
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
gcc -fPIC -shared input/iconv_wrapper.c -o libiconv_wrapper.so
cd charset/
rm -f aliases.rs
rustc aliases_gen.rs
./aliases_gen
cd ..
rustc -L . parserutils.rc --link-args -liconv_wrapper
