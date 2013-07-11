all: 
	rm -f *.so
	rustc -O wapcaplet.rs

debug:
	rm -f *.so
	rustc -Z verbose -Z debug-info wapcaplet.rs
	
clean:
	rm -f *.so

