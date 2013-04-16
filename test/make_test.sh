#! /bin/sh

# do clean 
rm -rf *.so *.o test_aliases test_parserutils_filter test_parserutils_inputstream test_parserutils test_riconv test_wapcaplet

# make test executables
export LD_LIBRARY_PATH=.:./..:$LD_LIBRARY_PATH
rustc test.rc
rustc -L . -L .. test_aliases.rs
rustc -L . -L .. test_parserutils_filter.rs
rustc -L . -L .. test_parserutils_inputstream.rs
rustc -L . -L .. test_parserutils.rs
rustc -L . -L .. test_riconv.rs
rustc -L . -L .. test_wapcaplet.rs
rustc -L . -L .. test_csdtect.rs