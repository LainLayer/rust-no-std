main: main.o
	ld -o main main.o -static

main.o: main.rs
	rustc main.rs -C panic="abort" --emit=obj
