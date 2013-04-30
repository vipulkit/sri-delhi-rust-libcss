#! /bin/sh
rm -rf *.so *.o 
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
gcc -fPIC -shared iconv_wrapper.c -o libiconv_wrapper.so 
rustc -L . iconv_crate.rc 
rustc -L . parserutils.rs
rustc -L . csdetect.rs
rustc -L . parserutils_filter.rs
rustc -L . parserutils_inputstream.rs
rustc -L . wapcaplet.rs
rustc -L . css_propstrings.rs
rustc -L . css_propstrings_parallel.rs
rustc -L . css_enum.rs
rustc -L . css_fpmath.rs
rustc -L . css_bytecode.rs
#rustc -L . css_ds.rs
rustc -L . css_lexer.rs
rustc -L . css_stylesheet.rs
rustc -L . css_language.rs
#rustc -L . css_parser.rs
#rustc -L . css_parse_properties.rs
#rustc -L . css_fontface.rs
#rustc -L . css.rs
rustc -L . css_select_const.rs
rustc -L . css_select_computed.rs
rustc -L . css_select_propget.rs

