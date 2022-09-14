# Rusty enigma machine
> Still playing around (⌒‿⌒)

## Getting started with Rust

Install Rust on your local machine, to do so please follow the official documentation

https://www.rust-lang.org/learn/get-started


### Get a local copy using git

```bash
git clone https://stash.mifinity.com/scm/devops/getpassword.git
```

## Build or Run this project:

build your project with cargo build, run the following in your local repository directory:
```bash
// Run you project:
cargo run
//test your project with
cargo test
//build documentation for your project with
cargo doc --open --no-deps
//publish a library to crates.io with
cargo publish
// build a release version with
cargo build --release
```

You should be able to compile and build this code on either MacOS, Lin or Windows.
However, all tests have been performed only for Linux.

Build and documentation will be created under ./target/
I have big doubts this will ever work on Windows... 

## Features


Providing the following:

- [Stanford, the Enigma machine] (https://web.stanford.edu/class/cs106j/handouts/36-TheEnigmaMachine.pdf)
- [Code & ciphers #Rotorspec] (https://www.codesandciphers.org.uk/enigma/rotorspec.htm)
- [Code & ciphers #ex] (https://www.codesandciphers.org.uk/enigma/example1.htm)


3 ROTORS selected out of 5 in stock  
Display the keys that are typed on keyboard + permutation  


- FAST ROTOR: Rotor on RIGHT is the one getting input from keyboard, Advances one each key stroke
- MEDIUM ROTOR: in the middle, advances evert 26 key stroke
- SLOW ROTOR: advances once evert 676 key stroke

When typing a letter the rotor settings ie JLY will move to JLZ  
Reflector implement a fixed permutation  
so fast rotor > medium > slow > reflector > slow > medium > fast > output  
Unsure why medium rotor will move during the operation! or just at the end.  


 
```
INPUT		A	B	C	D	E	F	G	H	I	J	K	L	M	N	O	P	Q	R	S	T	U	V	W	X	Y	Z
Rotor I		E	K	M	F	L	G	D	Q	V	Z	N	T	O	W	Y	H	X	U	S	P	A	I	B	R	C	J
Rotor II	A	J	D	K	S	I	R	U	X	B	L	H	W	T	M	C	Q	G	Z	N	P	Y	F	V	O	E
Rotor III	B	D	F	H	J	L	C	P	R	T	X	V	Z	N	Y	E	I	W	G	A	K	M	U	S	Q	O
Rotor IV	E	S	O	V	P	Z	J	A	Y	Q	U	I	R	H	X	L	N	F	T	G	K	D	C	M	W	B
Rotor V		V	Z	B	R	G	I	T	Y	U	P	S	D	N	H	L	X	A	W	M	J	Q	O	F	E	C	K
reflector C	(AF) (BV) (CP) (DJ) (EI) (GO) (HY) (KR) (LZ) (MX) (NW) (TQ) (SU)
```

## Todo
[ ] Rotor advancements as follow:

> The right rotor advances on every character and is therefore called the fast rotor. The
> middle rotor advances once every 26 characters and is called the medium rotor. The left
> rotor advances only once every 676 (26 × 26) characters and is called the slow rotor. 


## Contributing

## Related projects

## Licensing

not much here neither
