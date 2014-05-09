

all: emulator

emulator: emulator.rs
	rustc $^
