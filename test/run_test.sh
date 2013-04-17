
# run test programs 

#./test_aliases 
#./test_wapcaplet
#./test_riconv
#./test_parserutils 
#./test_parserutils_filter 
#./test_parserutils_inputstream ./utf16.txt 

./test_csdtect   utf32.txt
./test_csdtect   utf16.txt
./test_csdtect   utf8.txt
./test_csdtect   eucjp.txt 
./test_csdtect   chinese.txt 

chmod 777 *.csv
