use std::io::*;
//const rotors: [&str; 5] = ["EKMFLGDQVZNTOWYHXUSPAIBRCJ", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "BDFHJLCPRTXVZNYEIWGAKMUSQO", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];
const ROTORS: [&str; 5] = ["BDFHJLCPRTXVZNYEIWGAKMUSQO", "AJDKSIRUXBLHWTMCQGZNPYFVOE", "EKMFLGDQVZNTOWYHXUSPAIBRCJ", "ESOVPZJAYQUIRHXLNFTGKDCMWB", "VZBRGITYUPSDNHLXAWMJQOFECK"];

//const reflector: &str = "AFBVCPDJEIGOHYKRLZMXNWTQSU";
const REFLECTOR: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";

fn main() {
    println!("Hey there");
    let mut pickedrotors: Vec<usize> = Vec::new();
    let mut pickedsettings: Vec<char> = Vec::new();

    // Select rotors to use out of the 5 available
    for n in 1..4 {
        println!("Select rotor [type: digit 0-4] #{}:", n.to_string());
        pickedrotors.push(promptdata().parse::<usize>().unwrap());
    }

    // Select settings for the rotors
    for n in 1..4 {
        println!("Select your current setting for rotor [type char] #{}", n.to_string());
        pickedsettings.push(promptdata().chars().nth(0).unwrap());
    }

    println!("Text you wish to encrypt?");
    let originaltext = promptdata();
    let mut outstring = originaltext.to_lowercase();
    println!(".{:->100}", ".");

    //println!("INPUT:");
    //println!("{:>10} ", outstring);
    println!("Selected rotors: {:?} with settings: {:?}", &pickedrotors, &pickedsettings);


    // run the logic to encrypt the entire text / loop through char 
    for n in 0..outstring.chars().count() {                                                // looping in text to encrypt
        if outstring.chars().nth(n).unwrap().is_alphabetic() {                             // running only on alphabetic characters  
            outstring = encrypt(
                &outstring,                                                                // working string
                &mut pickedrotors,                                                         // selected rotors
                n,                                                                         // char position we want to translate    
                &mut pickedsettings.iter().map(|c| *c as usize - 97).collect::<Vec<_>>()   // this is the pickedsettings converted to usize vector instead of char
                );

            // ticking the 3 rotors fast = each time, medium = every 26 times, slow = 676 every times
            // fast rotor 
            if (pickedsettings[0] as u8 + 1) < 123 { pickedsettings[0] = (pickedsettings[0] as u8 + 1) as char; }
                else { pickedsettings[0] = 'a'; }
            // medium rotor
            if n % 26 == 0 && n != 0 { if (pickedsettings[1] as u8 + 1) < 123 { pickedsettings[1] = (pickedsettings[1] as u8 + 1) as char; }
                else { pickedsettings[1] = 'a'; }}
            // slow rotor
            if n % 676 == 0 && n != 0 {  if (pickedsettings[2] as u8 + 1) < 123 { pickedsettings[2] = (pickedsettings[2] as u8 + 1) as char; }
                else { pickedsettings[2] = 'a'; }}
        }
    }
    
    // show a bit of text and say bye
    //println!(".{:->100}", ".");
    println!("Final state or settings: {:?}", &pickedsettings);
    println!(".{:->100}", ".");
    println!("OUTPUT:");
    println!("{:>10} ", outstring);
    println!(".{:->100}", ".");

    println!("~ Bye now ...  (ᵔ◡ᵔ)ﾉ");    
}

fn encrypt(clear: &String, myrotor: &mut Vec<usize>, inc: usize, rotating: &mut Vec<usize>) -> String {
    //println!("{:?}", rotors);
    let mut workingstring: String = String::from(clear);
    
    //1. Looping in my 3 rotors right to left (fast, medium and slow)
    for n in 0..3 {
        workingstring.replace_range((inc)..(inc+1),                                   // replace char at pos inc to inc+1
            moverotor(ROTORS[myrotor[n]].to_lowercase(), rotating[n] as u8).chars().  // fetch rotated rotor
                nth(String::from_iter(('a'..='z').into_iter().collect::<Vec<char>>()) // index from alphabet
                    .find(workingstring.chars().nth(inc).unwrap()).unwrap())          // find char a-z at n position 
                .unwrap().to_string().as_str()                                        // replace range requires &str
        );
        //println!("#{}: Rotor: [{}] ... {} Rotating: {}", n.to_string(), ROTORS[myrotor[n]], workingstring, rotating[n]);        
    }

    //2. Mapping on static reflector
    workingstring.replace_range((inc)..(inc+1), 
        REFLECTOR.to_lowercase().                                                    // get my reflector in lcase
        chars().                                                                     // splice to char
        nth(((workingstring.chars().nth(inc).unwrap() as u32) - 97) as usize)        // get nth char at position matching char in string in unicode val
        .unwrap()                                                                    
        .to_string().as_str()
        );

    //3. Looping back in my 3 rotors left to right (slow, medium and fast)
    for n in (0..3).rev() {
        workingstring.replace_range((inc)..(inc+1),                                 // replace char at pos inc to inc+1
            (((
                moverotor(ROTORS[myrotor[n]].to_lowercase(), rotating[n] as u8).    // fetch rotated rotor
                find(workingstring.chars().nth(inc).unwrap())                       // find char in original string
                .unwrap() as u8) + 97)                                              // match with a-z
            as char).to_string().as_str()                                           // replace in original string
        );
        //println!("#{}: Rotor: [{}] ... {} Rotating: {}", n.to_string(), ROTORS[myrotor[n]], workingstring, rotating[n]);
    }
   workingstring.to_string()
}

fn moverotor(rotor: String, adv: u8) -> String {
    let mut rotated: String = "".to_string();
    for n in adv..26 {
        rotated = rotated + &rotor.chars().nth(n as usize).unwrap().to_string(); 
    }
    for n in 0..adv {
        rotated = rotated + &rotor.chars().nth(n as usize).unwrap().to_string();
    }
    rotated
}

fn promptdata() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    return input_string.trim().to_string();
}