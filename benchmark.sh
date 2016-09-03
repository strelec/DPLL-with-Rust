#!/bin/bash

sudo renice -n -19 $$

cargo build --release

for f in samples/trivial/* samples/easy/* samples/hard/*; do
	if [ -f $f ]; then
		echo
		echo $f:
		/usr/bin/time -f "%U\t%M" target/release/dpll-rust < $f > /dev/null
		/usr/bin/time -f "%U\t%M" target/release/dpll-rust < $f > /dev/null
	fi
done
