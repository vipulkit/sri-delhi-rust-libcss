#! /bin/sh

# do clean 
rm -rf *.so *.o test_aliases test_parserutils_filter test_parserutils_inputstream test_parserutils test_riconv test_wapcaplet

make test executables
export LD_LIBRARY_PATH=.:./..:$LD_LIBRARY_PATH
rustc test.rc

rustc -L ../libparserutils test_aliases.rs -L .
rustc -L ../libparserutils test_parserutils_filter.rs -L .
rustc -L ../libparserutils test_parserutils_inputstream.rs -L .
rustc -L ../libparserutils test_parserutils.rs -L .
#rustc -L ./.. test_riconv.rs -L .
rustc -L ../libwapcaplet test_wapcaplet.rs -L .
rustc -L ../libparserutils test_csdtect.rs -L .
rustc -L ../libcss -L ../libwapcaplet test_css_stylesheet.rs -L .
rustc -L ../libwapcaplet -L ../libcss propstrings_perf.rs -L .
rustc -L ../libparserutils -L ../libcss test_lexer_chunks.rs -L .
