# Rusty enigma machine
> Still playing around 	(ᵔ◡ᵔ)ﾉ

## Getting started with Rust

Install Rust on your local machine, to do so please follow the official documentation

[Rust get starteds](https://www.rust-lang.org/learn/get-started)


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
- [x] Rotor advancements as follow:
> The right rotor advances on every character and is therefore called the fast rotor. The
> middle rotor advances once every 26 characters and is called the medium rotor. The left
> rotor advances only once every 676 (26 × 26) characters and is called the slow rotor. 
- [x] Need to clean up variables and type with conversion all over the place 
- [ ] Should handle error if the input on rotor selection is not numeric and is not in range


## testing
Back & forth is working with several tests...

> Do not go gentle into that good night,(..)Old age should burn and rave at close of day;(..)Rage, rage against the dying of the light.(..)(..)Though wise men at their end know dark is right,(..)Because their words had forked no lightning they(..)Do not go gentle into that good night.(..)(..)Good men, the last wave by, crying how bright(..)Their frail deeds might have danced in a green bay,(..)Rage, rage against the dying of the light.(..)(..)Wild men who caught and sang the sun in flight,(..)And learn, too late, they grieved it on its way,(..)Do not go gentle into that good night.(..)(..)Grave men, near death, who see with blinding sight(..)Blind eyes could blaze like meteors and be gay,(..)Rage, rage against the dying of the light.(..)(..)And you, my father, there on the sad height,(..)Curse, bless, me now with your fierce tears, I pray.(..)Do not go gentle into that good night.(..)Rage, rage against the dying of the light.


0/1/2 with dfg
> mq pmy in evvctu xkzg erbr dzqa eoexb,(..)wbt yac npgnnp avcm pyf yzks ym dydrf ug cug;(..)vbff, nsyn zadwccf fov uzjzh qe sto jksqa.(..)(..)ewkfql vydx hug tk isstb ncw bchr szhz bb aewmi,(..)ujyqeez xixxg mhslt nio vnozsr ky isjjjquyc cqwc(..)mx wsx jw xvwcuh rvef lyps ogal otozx.(..)(..)ydav ydb, kwl csdl lpnm ad, xjtfzd zth jzfyti(..)wmjdz etlqd ypmlx paifb krlz yqgplk rz h zhxvp uth,(..)ynsj, jzbc muqlery fqw navft kp clu becei.(..)(..)gmht liv sii ajkoug zjq braj sea hho rt hrjqgp,(..)jrc pmzek, rbs wuza, buuq psfhypo zs po hkj trv,(..)sx wtq dr sniigh zowk svxe cjkz vgkps.(..)(..)bqclm kst, fmoq fcveg, ent nky lsmm atplodtb tbewd(..)dfuqg rzhu fltrg gndcr mgjg zxpdxuz zju kz prp,(..)sdxh, zbsm kopaytz dzm ldhxf wx ufg svsug.(..)(..)yvq gbb, vk nzoqqz, biksf wv qcv lcp ljdadv,(..)jjphs, jnivt, xn spj ezgm vsdw ehbozt ozpkm, p yqec.(..)kq pmx og jmgweb mumu ndsw niwk vbrpm.(..)vjom, nidl iyrjqwk cdk jpfwd dw xyi oxcjw.


0/3/1 with dfg
> vn mfp ah nqrlpv qocu nrea hiie zrpxx,(..)cct jhl rvkiuh xvwx iem mfdx sf xgwxf iv wxq;(..)bqat, uhzn uogadrz zgs rtgte wm znd fqzmd.(..)(..)kgxbpp mpbs fle ou soura xmu lxcp wqjz wl krnom,(..)pllhdjx btvbg ocppi rmi xlooqp ks iejvljmss yguj(..)et sge jj uikuia spch fytm iadf urbmh.(..)(..)shmp kxu, mav cjbf rteo pz, icsygw tix ibyrns(..)niycy ttuhv jfdcm suifd nnhw vdgqlk qt h zuxnh utg,(..)ytxn, wsyp rjdvwkc kpj zchzf kb yls welvx.(..)(..)sgzo hcz buq offsgo vjc xbit ovj wta nc javccy,(..)fly jqvmb, fjm ncij, fcpk qyuzowv yb ih cdc iqv,(..)jy xjq lr xjtzrz uumx bxmc wmkt dkkck.(..)(..)eipfu kyj, truh fctwd, jrv mor aodo ryflgbjz mselz(..)dynow shhz fjbxe lsbal ubrl acsicme emv xd uoq,(..)ddiz, wctm ijmaaur boj ijxvv wg dfg wjngv.(..)(..)ylc wqy, qh yzpljx, ydvsa ts kda dho mxcdpa,(..)vbizi, wiivn, rw zja kaxt etmm hxbxzw btqca, m xjyc.(..)hd cgx ia enrapg zfps mnir yhqv wewpx.(..)iupg, xiir joksuma ymt sadsb tr xoy gnrtz.



[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)