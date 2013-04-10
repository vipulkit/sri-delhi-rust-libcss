#! /bin/sh

# do clean 
rm -rf *.so *.o test_aliases test_parserutils_filter test_parserutils_inputstream test_parserutils test_riconv test_wapcaplet

# make test executables
export LD_LIBRARY_PATH=.:./..:$LD_LIBRARY_PATH
rustc -L ./.. test_aliases.rs
rustc -L ./.. test_parserutils_filter.rs
rustc -L ./.. test_parserutils_inputstream.rs
rustc -L ./.. test_parserutils.rs
rustc -L ./.. test_riconv.rs
rustc -L ./.. test_wapcaplet.rs


# run test programs 
./test_aliases 
./test_wapcaplet
./test_riconv
./test_parserutils 
./test_parserutils_filter 
./test_parserutils_inputstream ./utf16.txt 

 


