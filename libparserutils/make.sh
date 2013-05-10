rm -rf *.so *.o
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
gcc -fPIC -shared input/iconv_wrapper.c -o libiconv_wrapper.so
rustc -L . parserutils.rc
