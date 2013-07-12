all:
	rm -f aliases.rs
	rustc aliases_gen.rs

debug:
	rm -f aliases.rs
	rustc -Z verbose -Z debug-info aliases_gen.rs

clean:
	rm -f aliases.rs
	rm -f aliases_gen

run:
	./aliases_gen
