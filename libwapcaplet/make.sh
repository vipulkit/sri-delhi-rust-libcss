rm -f *.so
rustc -L . wapcaplet.rs --opt-level=2
rustc -L . wapcaplet2.rs --opt-level=2