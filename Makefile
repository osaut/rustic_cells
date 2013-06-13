RUST=rustc

all: exe


exe: *.rs
	$(RUST) --opt-level=3 -o ./cells run.rs


run: exe
	./run

test: *.rs
	$(RUST) run.rs --test -o cells-test
	./cells-test

clean:
	-rm ./cells ./cells-test *.vtk
