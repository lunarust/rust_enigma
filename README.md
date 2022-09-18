# Rusty enigma machine
> Still playing around 	(ᵔ◡ᵔ)

## Getting started with Rust

Install Rust on your local machine, to do so please follow the official documentation
[Rust get started](https://www.rust-lang.org/learn/get-started)


### Get a local copy using git

```bash
git clone https://github.com/lunarust/rust_enigma.git
```

## Build or Run this project:

build your project with cargo build, run the following in your local repository directory:
```bash
// Run you project:
cargo run
//test your project with
cargo test
//build documentation under ./target/ for your project with 
cargo doc --open --no-deps
//publish a library to crates.io with
cargo publish
// build a release version with
cargo build --release
```

You should be able to compile and build this code on either MacOS, Lin or Windows.
However, all tests have been performed using Linux.

## Features
Trying to simulate a enigma machine.

Providing the following:
- [Stanford, the Enigma machine](https://web.stanford.edu/class/cs106j/handouts/36-TheEnigmaMachine.pdf)
- [Code & ciphers #Rotorspec](https://www.codesandciphers.org.uk/enigma/rotorspec.htm)
- [Code & ciphers #ex](https://www.codesandciphers.org.uk/enigma/example1.htm)


3 ROTORS selected out of 5 in stock  
Display the keys that are typed on keyboard + permutation  

- FAST ROTOR: Rotor on RIGHT is the one getting input from keyboard, Advances one each key stroke
- MEDIUM ROTOR: in the middle, advances evert 26 key stroke
- SLOW ROTOR: advances once evert 676 key stroke

When typing a letter the rotor settings ie JLY will move to JLZ  
Reflector implement a fixed permutation  
so fast rotor > medium > slow > reflector > slow > medium > fast > output  

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
- [x] Rotor ticks as follow:
> The right rotor advances on every character and is therefore called the fast rotor. The middle rotor advances once every 26 characters and is called the medium rotor. The left rotor advances only once every 676 (26 × 26) characters and is called the slow rotor. 
- [x] Need to clean up variables and type with conversion all over the place 
- [ ] Should handle error if the input on rotor selection is not numeric and is not in range


## Testing with Dylan Thomas
Works back & forth ...
Guess they would strip their text from anything non alphabetical. Replacing \n by (..) and skipping anything not in [[:alpha:]]
Returning text all in lower case, guessing it would all be upper or lower.

Text you wish to encrypt?
> Do not go gentle into that good night,(..)Old age should burn and rave at close of day;(..)Rage, rage against the dying of the light.(..)(..)Though wise men at their end know dark is right,(..)Because their words had forked no lightning they(..)Do not go gentle into that good night.(..)(..)Good men, the last wave by, crying how bright(..)Their frail deeds might have danced in a green bay,(..)Rage, rage against the dying of the light.(..)(..)Wild men who caught and sang the sun in flight,(..)And learn, too late, they grieved it on its way,(..)Do not go gentle into that good night.(..)(..)Grave men, near death, who see with blinding sight(..)Blind eyes could blaze like meteors and be gay,(..)Rage, rage against the dying of the light.(..)(..)And you, my father, there on the sad height,(..)Curse, bless, me now with your fierce tears, I pray.(..)Do not go gentle into that good night.(..)Rage, rage against the dying of the light.
> .---------------------------------------------------------------------------------------------------.
> Selected rotors: [1, 2, 3] with settings: ['a', 'a', 'a']
> .---------------------------------------------------------------------------------------------------.
> Final state or settings: ['p', 'x', 'b']
> .---------------------------------------------------------------------------------------------------.
> OUTPUT:
> fa zvq br dafvdj dbxm wiqq urjr jmrrl,(..)dqn lob uwnlwn aknw ozo djjd kl gcrbu mq jpg;(..)vlag, nsvs uhkohrz bzf eiuvs pu end fjcgh.(..)(..)kykcpl vejl taw ha kpsza kem rrvd xzkd py xecqn,(..)inywdel xinck pvgwt jtx mnthvu pm nptmayohd sxsx(..)sn mes qy msoimq mmqd mdly fhjc xfnwj.(..)(..)zzww ujg, isc hcpm plxi kn, yzqqfo pgc vvaoqi(..)kwwal oclet zkazk kqrjx fman muxlhg za d qloqe lkp,(..)uvwz, ikps qwuczco jyv pdvvt ra bvi xlpzh.(..)(..)rasp vwf fcw tigyug jqq atac czn ghi qs kqdsqc,(..)iem cwjef, kbg iiyn, luqg cwyfxzy nr ji faz mfv,(..)yk jnq kr lxyowf bsvk rkqc tzhq uqrpk.(..)(..)riowl eay, pkwb pwewf, cpg imc qmpz lrffgmyk geybx(..)njksc ijpl ncgni qefet whwx qaluwmf qft oo oio,(..)psqz, ojdz sijzibz lmb awdfb je wxp xhhzh.(..)(..)qkv fcy, qd hppllp, xdkca sr wdj wpi cdodti,(..)oweuq, ozjdg, oq ycl lgis vbgi mvtorj hytvx, e wnvk.(..)zu tjf yb matbrs smxa hauz fcgc jmtvz.(..)bqcw, koud wabuvoo vgj ianpl ma vca egeki. 
> .---------------------------------------------------------------------------------------------------.
> ~ Bye now ...  (ᵔ◡ᵔ)ﾉ

## Yet another transcript to test
> .---------------------------------------------------------------------------------------------------.
> Selected rotors: [3, 2, 1] with settings: ['w', 'f', 'g']
> OUTPUT: 
> kxrxou: ryne, rndo cmx vjb, acod? (..) kprppx wrvlx: v'p dwrspm! del im qudqa g omtl rfbz upolysdjwn zeiltx, bgq xjyfb ocsa-b?! (..) tqgnkez: jcwg deo jbz lblrj fz pkxiloh?  (..) raaxtr qtfsy: vroc tpia axs ktlfdsmt! 


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)