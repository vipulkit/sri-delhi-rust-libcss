#! /bin/sh
rm -f *.so
rustc -L . -L ../libwapcaplet -L ../libparserutils css.rc
