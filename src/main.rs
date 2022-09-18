use std::io::*;
use std::env;
// donotgogentleintothatgoodnight
//const rotors: [&str; 5] = ["EKMFLGDQVZNTOWYHXUSPAIBRCJ", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "BDFHJLCPRTXVZNYEIWGAKMUSQO", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];
const ROTORS: [&str; 5] = ["BDFHJLCPRTXVZNYEIWGAKMUSQO", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];

//const reflector: &str = "AFBVCPDJEIGOHYKRLZMXNWTQSU";
const REFLECTOR: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Hey there");
    let mut pickedrotors: Vec<usize> = Vec::new();
    let mut pickedsettings: Vec<char> = Vec::new();

    // Select rotors to use out of the 5 available
    for n in 1..4 {
        println!("Select rotor [type: digit 1-5] #{}:", n.to_string());
        pickedrotors.push(promptdata().parse::<usize>().unwrap() -1);
    }

    // Select settings for the rotors
    for n in 1..4 {
        println!("Select your current setting for rotor [type char] #{}", n.to_string());
        pickedsettings.push(promptdata().chars().nth(0).unwrap());
    }

    println!("List of selected rotors: {:?} with setting: {:?}", &pickedrotors, &pickedsettings);
    
    println!("Text you wish to encrypt?");
    let originaltext = promptdata();
    let mut outstring = originaltext.to_lowercase();
    println!("-------------------");

    println!("INPUT: {:>50} ", outstring);

    // run the logic to encrypt the entire text / loop through char 
    for n in 0..outstring.chars().count() {
        // running enigma only on alphabetic characters
        if outstring.chars().nth(n).unwrap().is_alphabetic() {  
            outstring = encrypt(&outstring, &mut pickedrotors, n, pickedsettings[0], pickedsettings[1], pickedsettings[2]); 

        if (pickedsettings[0] as u8 + 1) < 123 { pickedsettings[0] = (pickedsettings[0] as u8 + 1) as char; }
        else { pickedsettings[0] = 'a'; }

        if n % 26 == 0 && n != 0 { if (pickedsettings[1] as u8 + 1) < 123 { pickedsettings[1] = (pickedsettings[1] as u8 + 1) as char; }
            else { pickedsettings[1] = 'a'; }}
        if n % 676 == 0 && n != 0 {  if (pickedsettings[2] as u8 + 1) < 123 { pickedsettings[2] = (pickedsettings[2] as u8 + 1) as char; }
            else { pickedsettings[2] = 'a'; }}
        }
    }
    
    // show a bit of text and say bye
    println!("");
    println!("");
    println!("OUTPUT: {:>49} ", outstring);
    println!("-------------------");
    println!("Based on Rotors: {:?}, Settings: {:?}", pickedrotors, pickedsettings);
    println!("-------------------");
    println!("~ Bye now ... // (ᵔ◡ᵔ)~");    
}

fn encrypt(clear: &String, myrotor: &mut Vec<usize>, inc: usize, setone: char, settwo: char, setthree: char) -> String {
    let mut rotating: Vec<usize> = Vec::new();
    rotating.push((setone as usize) - 97);
    rotating.push((settwo as usize) - 97);
    rotating.push((setthree as usize) - 97);
    let myalphabet = ('a'..='z').into_iter().collect::<Vec<char>>();
    
    //println!("{:?}", rotors);
    let mut workingstring: String = String::from(clear);
    
    let charrep: String;
    //1. Looping in my 3 rotors right to left (fast, medium and slow)
    for n in 0..3 {
        //println!("ROTATING first LOOP {} - {}", n, rotating[n]);

        workingstring.replace_range((inc)..(inc+1), 
            moverotor(ROTORS[myrotor[n]].to_lowercase(), rotating[n] as u8)
                .chars().nth(String::from_iter(myalphabet.clone()).find(workingstring.chars().nth(inc).unwrap()).unwrap()).unwrap().to_string().as_str()
        );
        //println!("#{}: Rotor: [{}] ... {} Rotating: {}", n.to_string(), ROTORS[myrotor[n]], workingstring, rotating[n]);
    }

    //2. Mapping on static reflector
    charrep = REFLECTOR.to_lowercase().chars().nth(((workingstring.chars().nth(inc).unwrap() as u32) - 97) as usize).unwrap().to_string();
    workingstring.replace_range((inc)..(inc+1), &charrep);

    //println!("#..: reflector: [{}] . {} .. {}", REFLECTOR, workingstring, charrep);

    //3. Looping back in my 3 rotors left to right (slow, medium and fast)
    for n in (0..3).rev() {
        //println!("ROTATING second LOOP {} - {}", n, rotating[n]);
        //charrep = (((ROTORS[myrotor[n]].to_lowercase().find(workingstring.chars().nth(inc).unwrap()).unwrap() as u8) + 97) as char).to_string();
        workingstring.replace_range((inc)..(inc+1), 
            (((
                moverotor(ROTORS[myrotor[n]].to_lowercase(), rotating[n] as u8).
                find(workingstring.chars().nth(inc).unwrap())
                .unwrap() as u8) + 97) 
            as char).to_string().as_str()           
        );

        //println!("#{}: Rotor: [{}] ... {} Rotating: {}", n.to_string(), ROTORS[myrotor[n]], workingstring, rotating[n]);
    }
   //println!("#{} Settings: {}-{}-{} .. {}", inc.to_string(), setone.to_string(), settwo.to_string(), setthree.to_string(), workingstring);
   workingstring.to_string()
}

fn moverotor(rotor: String, adv: u8) -> String {
    
    //let rotations = rotor.find(((adv as char) as u8) as char).unwrap();
    //println!("###rotor {} adv {} {} ", rotor, adv, adv as char);

    //let mut rotated: Vec<char> = Vec::new();
    let mut rotated: String = "".to_string();
    // post is 8 then 8 = 0, whatever is before comes at the end
    for n in adv..26 {
        rotated = rotated + &rotor.chars().nth(n as usize).unwrap().to_string(); 
    }
    for n in 0..adv {
        rotated = rotated + &rotor.chars().nth(n as usize).unwrap().to_string();
    }
    //println!("original: {:?}, rotated: {:?}", rotor, rotated);
    //rotated.into_iter().map(|i| i.to_string()).collect::<String>()
    rotated
}
fn promptdata() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    return input_string.trim().to_string();
}
