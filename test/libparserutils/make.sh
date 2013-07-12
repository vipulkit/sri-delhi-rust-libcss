rm *filter 
cp ./../../libparserutils/*.so .
rustc --test -L ../../libparserutils filter.rs  --link-args -liconv_wrapper -L./../../libparserutils/
