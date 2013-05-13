RUST=rust

all: exe


exe: *.rs 
	$(RUST) build --opt-level=3 run.rs


run: exe
	./run

test: *.rs
	$(RUST) test run.rs 

clean:
	-rm ./run ./runtest~ *.vtk	
