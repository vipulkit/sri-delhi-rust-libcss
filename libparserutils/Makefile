all: 
	rm -rf *.so *.o
	export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
	gcc -fPIC -shared input/iconv_wrapper.c -o libiconv_wrapper.so
	cd charset;	make; make run;	cd ..
	rustc -O -L . parserutils.rc --link-args -liconv_wrapper

debug: 
	rm -rf *.so *.o
	export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
	gcc -fPIC -shared input/iconv_wrapper.c -o libiconv_wrapper.so
	cd charset;	make; make run;	cd ..
	rustc -Z verbose -Z debug-info  -L . parserutils.rc --link-args -liconv_wrapper

clean: 
	rm -rf *.so *.o
	cd charset;	make; make clean;	cd ..
