#! /bin/sh

# do clean 
rm -rf *.so *.o test_aliases test_parserutils_filter test_parserutils_inputstream test_parserutils test_riconv test_wapcaplet

make test executables
export LD_LIBRARY_PATH=.:./..:$LD_LIBRARY_PATH
rustc test.rc

rustc test_aliases.rs -L . -L ../libparserutils
rustc test_parserutils_filter.rs -L . -L ../libparserutils
rustc test_parserutils_inputstream.rs -L . -L ../libparserutils
rustc test_parserutils.rs -L . -L ../libparserutils
#rustc -L ./.. test_riconv.rs -L .
rustc test_wapcaplet.rs -L . -L ../libwapcaplet
rustc test_csdtect.rs -L . -L ../libparserutils
rustc test_css_stylesheet.rs -L . -L ../libcss -L ../libwapcaplet -L ../libparserutils/
rustc propstrings_perf.rs -L . -L ../libwapcaplet -L ../libcss -L ../libparserutils/
rustc test_lexer_chunks.rs -L . -L ../libparserutils -L ../libcss -L ../libwapcaplet
rustc test_lexer_chunks_file.rs -L . -L ../libparserutils -L ../libcss -L ../libwapcaplet
cd libcss
./make_test.sh
cd ..