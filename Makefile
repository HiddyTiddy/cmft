

in_files := $(wildcard ./examples/*.in)


bin:
	cargo build

test: $(in_files)
	make bin
	cat $^ | ./target/debug/cmft | diff - $(subst .in,.out, $^) 
