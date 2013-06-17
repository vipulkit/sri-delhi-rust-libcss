rustc -L ../../libwapcaplet/ perf-test.rs --opt-level=2
rustc -L ../../libwapcaplet/ wapcaplet2_perf_test.rs --opt-level=2
rustc -L ../../libwapcaplet/ wapcaplet2_perf_test_thread_safe.rs --opt-level=2 
rustc --test -L ../../libwapcaplet/ wapcaplet2_basictests.rs --opt-level=2 
