all:
	export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
	gcc -fPIC -shared libparserutils/input/iconv_wrapper.c -o libiconv_wrapper.so
	cd libparserutils/charset;	make; make run;	cd ..
	cd parse/properties; make; make run; cd -
	rustc -O -L . css.rc --link-args -liconv_wrapper

debug:
	cd parse/properties; make; make run; cd -
	rustc -Z verbose -Z debug-info -L ../libwapcaplet -L ../libparserutils css.rc

clean:
	rm -f *.so
	cd parse/properties; make clean; cd -