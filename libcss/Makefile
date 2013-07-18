all:
	cd parse/properties; make; make run; cd -
	rustc -O -L ../libwapcaplet -L ../libparserutils css.rc

debug:
	cd parse/properties; make; make run; cd -
	rustc -Z verbose -Z debug-info -L ../libwapcaplet -L ../libparserutils css.rc

clean:
	rm -f *.so
	cd parse/properties; make clean; cd -