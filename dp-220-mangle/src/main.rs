#![feature(collections)]
fn main() {
    let sentences = vec![ "This challenge doesn't seem so hard.",
                          "There are more things between heaven and earth, Horatio, than are dreamt of in your philosophy.",
                          "Eye of Newt, and Toe of Frog, Wool of Bat, and Tongue of Dog.",
                          "Adder's fork, and Blind-worm's sting, Lizard's leg, and Howlet's wing.",
                          "For a charm of powerful trouble, like a hell-broth boil and bubble.",
                          "Scrooge McDuck."];
    for s in sentences {
        mangle(s);
    }
}

fn mangle(input: &str) {
    println!("Input: {}", input);
    let s: Vec<&str> = input.split(' ').collect();                            // Separate each word into its own Vector

    print!("Output: ");
    for x in 0..s.len() {
        let chars: Vec<char> = s[x].chars().collect();                        // Create a vector that we will compare to later...
        let mut sorted: Vec<char> = s[x].to_lowercase().chars().collect();    // Create a vector to sort and print out
        sorted.sort();                                                        // Sorting...
        sorted.retain(|&c| c.is_alphanumeric());                              // Dropping all non-alphanumeric characters.

        for y in 0..sorted.len() {
            if chars[y].is_uppercase() {                                      // If any character of a word in the original is uppercase, so should this character.
                print!("{}", sorted[y].to_uppercase().next().unwrap());
            } else if ! chars[y].is_alphanumeric() {                          // If the original word had punctuation, we insert at the correct position
                print!("{}{}", chars[y], sorted[y]);
            } else {
                print!("{}", sorted[y]);                                      // Otherwise, we just print the letter.
            }
        }
        if x + 1 == s.len() {                                                 // Finally, we add back our spaces / punctuation
            print!(".")
        } else {
            print!(" ");
        }
    }
    println!("\n");
}
