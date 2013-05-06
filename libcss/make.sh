#! /bin/sh
rm -f *.so
rustc css_fpmath.rs 
rustc css_enum.rs 
rustc -L . css_bytecode.rs 
rustc -L. -L ../libwapcaplet css_propstrings.rs
rustc -L. -L ../libwapcaplet css_propstrings_parallel.rs
rustc -L . -L ../libparserutils/ css_lexer.rs
rustc -L . css_stylesheet.rs 
rustc -L . -L ../libwapcaplet css_properties.rs
rustc -L . -L ../libwapcaplet css_language.rs 
rustc -L . css_select_const.rs 
rustc -L . css_select_propset.rs
rustc -L . css_select_computed.rs

