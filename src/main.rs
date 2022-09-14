use std::env;
use std::io::*;
//const rotors: [&str; 5] = ["EKMFLGDQVZNTOWYHXUSPAIBRCJ", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "BDFHJLCPRTXVZNYEIWGAKMUSQO", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];
const rotors: [&str; 5] = ["BDFHJLCPRTXVZNYEIWGAKMUSQO", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];

//const reflector: &str = "AFBVCPDJEIGOHYKRLZMXNWTQSU";
const reflector: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";

fn main() {
    println!("Hey there");
    let mut pickedrotors: Vec<usize> = Vec::new();
    let mut pickedsettings = Vec::new();

    for n in 1..4 {
        println!("Select rotor #{}:", n.to_string());
        pickedrotors.push(promptdata().parse::<usize>().unwrap() -1);
    }

    for n in 1..4 {

        println!("Select your current setting for rotor #{}", n.to_string());
        pickedsettings.push(promptdata());
    }

    println!("List of selected rotors: {:?} with setting: {:?}", &pickedrotors, &pickedsettings);
    
    println!("Text you wish to encrypt?");
    let originaltext = promptdata();
    let mut outstring = String::from(originaltext);

    println!("INPUT: {:>50} ", outstring);

    for n in 0..outstring.chars().count() { outstring = encrypt(&outstring, &outstring, &mut pickedrotors, n, "q", "a", "v"); }
    
    println!("OUTPUT: {:>49} ", outstring);
    println!("-------------------");
    println!("~ Bye now ~~  (ᵔ◡ᵔ)/");    
}

fn encrypt(clear: &String, cyph: &str, myrotor: &mut Vec<usize>, inc: usize, setone: &str, settwo: &str, setthree: &str) -> String {
    //println!("{:?}", rotors);
    let mut workingstring: String = String::from(clear).to_lowercase();
    let mut charrep: String = "O".to_string();

    // Looping in my 3 rotors (fast, medium and slow)
    for n in 0..3 {
        charrep = rotors[myrotor[n]].chars().nth(((workingstring.to_lowercase().chars().nth(inc).unwrap() as u32) - 97) as usize).unwrap().to_string();
        workingstring.replace_range((inc)..(inc+1), &charrep);

        //println!("#{}: Rotor: [{}] . {} .. {}", n.to_string(), rotors[myrotor[n]], workingstring, charrep);
    }

    charrep = reflector.chars().nth(((workingstring.to_lowercase().chars().nth(inc).unwrap() as u32) - 97) as usize).unwrap().to_string();
    workingstring.replace_range((inc)..(inc+1), &charrep);
    //println!("#..: reflector: [{}] . {} .. {}", reflector, workingstring, charrep);

    for n in (0..3).rev() {
        charrep = (((rotors[myrotor[n]].to_lowercase().find(workingstring.to_lowercase().chars().nth(inc).unwrap()).unwrap() as u8) + 97) as char).to_string();
        workingstring.replace_range((inc)..(inc+1), &charrep);

        //println!("#{}: Rotor: [{}] . {} .. {}", n.to_string(), rotors[myrotor[n]], workingstring, charrep);
    }

    workingstring.to_string()
}
fn promptdata() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    return input_string.trim().to_string();
}