rm -f *.so
rustc -L . wapcaplet.rs --opt-level=2
