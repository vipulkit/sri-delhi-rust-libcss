all:
	rm -f *.so
	cd parse/properties; make; cd -
	rustc -O -L ../libwapcaplet -L ../libparserutils css.rc

debug:
	rm -f *.so
	cd parse/properties; make; make run; make clean; cd -
	rustc -Z verbose -Z debug-info -L ../libwapcaplet -L ../libparserutils css.rc

clean:
	rm -f *.so
	cd parse/properties; make clean; cd -