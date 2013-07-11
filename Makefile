all:
	cd libwapcaplet;	make;	cd ..
	cd libparserutils;	make;	cd ..
	cd libcss;	make;	cd ..

debug:
	cd libwapcaplet;	make debug;	cd ..
	cd libparserutils;	make debug;	cd ..
	cd libcss;	make debug;	cd ..

test:
	cd test;	make;	cd ..
	
test-debug:
	cd test;	make debug;	cd ..
	
clean:
	cd libwapcaplet;	make clean;	cd ..
	cd libparserutils;	make clean;	cd ..
	cd libcss;	make clean;	cd ..
	cd test;	make clean;	cd ..
	
