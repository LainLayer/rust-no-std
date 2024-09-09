main: main.o
	ld -o main main.o -static --nmagic
	strip -s main

main.o: main.rs
	rustc main.rs --emit=obj \
                  -C opt-level=z \
                  -C panic="abort" \
                  -C strip="debuginfo"

clean:
	rm main.o main
