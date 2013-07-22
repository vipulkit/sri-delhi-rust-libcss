all:
	rustc test_data.rs
	rustc test_data_avg.rs

debug: 
	rustc  -Z verbose -Z debug-info test_data.rs
	rustc  -Z verbose -Z debug-info test_data_avg.rs

clean:
	rm -f test_data
	rm -f select-auto
	rm -f test_data_avg
	rm -f c_output.txt
	rm -f r_output.txt
	
run:
	./perform_test_avg