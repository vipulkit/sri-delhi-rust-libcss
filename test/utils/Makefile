all:
	rustc test_data.rs

debug: 
	rustc  -Z verbose -Z debug-info test_data.rs

clean:
	rm -f test_data
	rm -f select-auto
	
run:
	./test_data
	./select-auto
	
	