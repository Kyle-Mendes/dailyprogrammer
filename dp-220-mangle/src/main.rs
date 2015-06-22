#![feature(collections)] 

fn main() {
    let sentances = vec![ "This challenge doesn't seem so hard.",
                          "There are more things between heaven and earth, Horatio, than are dreamt of in your philosophy.",
                          "Eye of Newt, and Toe of Frog, Wool of Bat, and Tongue of Dog.",
                          "Adder's fork, and Blind-worm's sting, Lizard's leg, and Howlet's wing.",
                          "For a charm of powerful trouble, like a hell-broth boil and bubble."];
    for s in sentances {
        mangle(s);
    }
}

fn mangle(input: &str) { 
    println!("Input: {}", input);
    let s: Vec<&str> = input.split(' ').collect();

    print!("Output: ");
    for x in 0..s.len() {
        let chars: Vec<char> = s[x].chars().collect();
        let mut sorted: Vec<char> = s[x].to_lowercase().chars().collect();
        sorted.sort();
        sorted.retain(|&c| c.is_alphanumeric());

        for y in 0..sorted.len() {
            if y == 0 && chars[0].is_uppercase() {
                print!("{}", sorted[y].to_uppercase().next().unwrap()); 
            } else if ! chars[y].is_alphanumeric() {
                print!("{}{}", chars[y], sorted[y]); 
            } else { 
                print!("{}", sorted[y]); 
            }
        }
        if x + 1 == s.len() { 
            print!(".")
        } else {
            print!(" ");
        }
    }
    println!("\n");
}
