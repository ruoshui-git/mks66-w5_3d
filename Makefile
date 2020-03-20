run:
	cargo test -- --nocapture script

gallery:
	cargo run --release
	convert -delay 10 img{1..9}.ppm img{8..2}.ppm perspectives.gif


clean:
	rm -f *.ppm *.gif *.png
	cargo clean