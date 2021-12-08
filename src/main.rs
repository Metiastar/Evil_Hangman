use std::{io::{Write, BufReader, BufRead}, fs::File};
use rand::{thread_rng, Rng};

/*TODO:
*   import dictionary.txt
*   prompt user for:
*       word length
*       how many guesses they get
*       whether they want words remaining in word list
*
*   main game print out:
*       how many guesses are remaining
*       what letters have already been guessed
*       how much of the word has been guessed already
*       congratulate player if they guess a letter and/or word correctly
*       if player runs out of guesses, computer chooses a word and displays it
*/
struct Hangman{
    word_database: Vec<String>,
    word_length: usize,
    initial_guesses: i32,
    guesses: i32,
    guessed_letters: Vec<String>,
    word: String,
}
impl Default for Hangman{
    fn default() -> Hangman{
        Hangman{
            word_database: Vec::new(),
            word_length: 4,
            initial_guesses: 10,
            guesses: 0,
            guessed_letters: Vec::new(),
            word: String::new(),
        }
    }
}
impl Hangman{

    fn add_guessed_letter(&mut self, letter:&str){
        self.guessed_letters.push(letter.to_string());
    }

    pub fn get_guesses_left(&self) -> i32{
        return self.initial_guesses - self.guesses;
    }

    pub fn set_initial_guesses(&mut self, initial:i32){
        self.initial_guesses = initial;
    }

    pub fn ask_initial_guesses(&mut self) -> i32{
        print!("How many guesses do you want to have?: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let answer = line.trim();

        let l = answer.parse::<i32>().expect("Could not parse first Row input");
        return l;
    }

    pub fn set_length(&mut self, len:usize){
        self.word_length = len;
    }

    pub fn ask_length(&mut self) -> i32{
        print!("How long do you want the word to be?: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let answer = line.trim();

        let l = answer.parse::<i32>().expect("Could not parse first Row input");
        
        return l;
    }

    //check to make sure there is a word of length requested
    pub fn check_length(&self, length:i32) -> bool{

        if length > 0{
            let reader = BufReader::new(File::open("src/dictionary.txt").expect("Can't read dictionary"));

            for line in reader.lines(){
                let word = line.unwrap();

                if word.chars().count() == length as usize{
                    return true
                }
            }
        }
        
        return false
    }

    pub fn make_init_word(&mut self){
        let word_vec = vec!["*".to_string(); self.word_length];
        self.word = word_vec.join("");
    }

    //check if player has won the game
    pub fn check_word_filled(&self) -> bool{
        if self.word.contains("*"){
            return false
        }else{
            return true
        }
    }

    //If player loses, choose random word
    pub fn choose_random_word(&self) -> String{
        let mut rng = thread_rng(); let mut word = String::new();
        let index = rng.gen_range(0..self.word_database.len());
        for (i, w) in self.word_database.iter().enumerate(){
            if i == index{
                word = w.to_string();
            }
        }
        return word
    }

    fn add_letter_to_word(&mut self, list:Vec<usize>, letter:&str){
        let mut new_word = self.word.to_string();
        let num: Vec<char> = self.word.chars().collect();
        for (i, _j) in num.iter().enumerate(){
            for (_i,j) in list.iter().enumerate(){
                if i == *j{
                    new_word.replace_range(i..(i+1), letter);
                }
            }
            
        }
        self.word = new_word.to_string();
        
    }

    pub fn make_word_database(&mut self){
        let reader = BufReader::new(File::open("src/dictionary.txt").expect("Can't read dictionary"));

        for line in reader.lines(){
            let word = line.unwrap();

            if word.chars().count() == self.word_length{
                self.word_database.push(word);
            }
        }

    }

    pub fn count_database(&self) -> usize{
        return self.word_database.len();
    }

    pub fn ask_show_database_count(&self) -> bool{
        print!("Do you wanna know how many words I could be thinking of after each guess?|| 1/T:yes please, anything else: nah || answer: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let answer = line.trim().to_lowercase();

        if answer.eq("1") || answer.eq("t") || answer.eq("yes") || answer.eq("y"){
            return true
        }else{
            return false
        }

    }

    /*Categorizes the remaining list of words based on how many have the guess or not
    * Returns true if list only has words comprised of the guess
    * Returns list without the guess in it
    */
    fn categorize(&mut self, guess:&str) -> (bool, Vec<String>){
        let list_length = self.word_database.len();
        let mut has_guess:usize = 0; let mut guess_bool = false;
        

        for w in &self.word_database{
            if w.contains(guess){
                has_guess += 1;
            }
        }

        let no_guess = list_length - has_guess;
        let mut non_guess_list = Vec::new();
        if no_guess > 0{//to eliminate the most amount of player guesses
            
            //get list of words without guess
            for (_i, word) in self.word_database.iter().enumerate(){
                if !word.contains(guess){
                    non_guess_list.push(word.to_string());
                }
            }

        }else{ //every word has the guess in it
            guess_bool = true
        }

        return (guess_bool, non_guess_list);
    }

    /*Find the lowest possible multiple of the guess to make it harder to understand what future letters are*/
    fn categorize_add_guess(&mut self, guess:&str) -> (usize, bool){
        let mut plural = false;
        
        let index_list = self.get_multiple_count(guess);

        //Compare the list of repeated indices to find the smallest amount repeated
        let mut smallest_index = 0; let mut smallest_found = false;
        for (_pos, v) in index_list.iter().enumerate(){//pos is index, v is value
            
            if !smallest_found && (v > &0) {
                smallest_index = *v;
                smallest_found = true;
            }
        }
        if smallest_index > 1{
            plural = true;
        }

        //remove words with more than the smallest repeated
        self.remove_multiples(guess, smallest_index);

        //find possible patterns for letter choice and choose largest of those groups
        let (patterns, count) = self.get_multiple_index_counts(guess);
        let mut biggest_pattern_count = 0; let mut biggest_pattern = Vec::new(); let mut temp_vec: Vec<Vec<usize>> = Vec::new();

        //iterate through the pattern counts first
        for (i, value) in count.iter().enumerate(){
            //iterate through the patterns until they match the index of the pattern count
            for (j, pattern) in patterns.iter().enumerate(){
                if j == i{

                    if *value > biggest_pattern_count{
                        if !temp_vec.is_empty(){
                            temp_vec.clear()
                        }
                        temp_vec.push(pattern.to_vec());
                        
                        biggest_pattern_count = *value;
        
                    }else if *value == biggest_pattern_count{
                        temp_vec.push(pattern.to_vec());
                    }

                }
            }
        }

        //if multiple biggest patterns, choose one at random
        if temp_vec.len() > 1{
            let mut rng = thread_rng();
            let pattern = rng.gen_range(0..(temp_vec.len()-1));
            let chosen_vec = &temp_vec[pattern];
            for i in chosen_vec{
                biggest_pattern.push(*i);
            }
        }else{
            for i in &temp_vec[0]{
                biggest_pattern.push(*i);
            }
        }

        //remove words that don't match this specific pattern
        let guess_byte = guess.as_bytes()[0]; let mut pattern_match = true; let mut remove_vec = Vec::new();//vec of words indices to remove
        for (i, word) in self.word_database.iter().enumerate(){

            for (_k, j) in biggest_pattern.iter().enumerate(){
                let c = word.as_bytes()[*j];
                if c != guess_byte{
                    pattern_match = false;
                }
            }

            if !pattern_match{
                remove_vec.push(i);
            }
            pattern_match = true;
        }

        for (_r, v) in remove_vec.iter().enumerate(){
            self.word_database.swap_remove(*v);
        }

        //add the letters to the word
        self.add_letter_to_word(biggest_pattern, guess);

        return (smallest_index, plural);
    }

    /*In each word: Gets how many times the guesed letter is repeated
    *
    * Returns vector with numbers corresponding to how many words repeated index# times
    * Vector index represents number of repeats starting with Vector[0] = 1
    *
    */
    fn get_multiple_count(&mut self, guess:&str) -> Vec<usize>{
        let mut multiple = vec![0; self.word_length];
        for (_i, word) in self.word_database.iter().enumerate(){
            for i in 0..self.word_length{//get index of each character in the word
                if word.as_bytes()[i] == guess.as_bytes()[0]{
                    multiple[i] += 1;
                }
            }
        }
        return multiple;
    }

    /*Gets all possible guess patterns and how frequent they are
    * Returns 2 vectors:
    *   First one holds the indice pattern
    *   Second holds the number of words using those indice patterns
    */
    fn get_multiple_index_counts(&self, guess:&str) -> (Vec<Vec<usize>>, Vec<i32>){
        /*Note for programmer:
        *   We do know that the remaining list only contains however many spots we need to fill
        */
        let letter = guess.as_bytes()[0];

        //initialize combination, combination count, and temp combination vectors
        let mut combination_vec = Vec::new();
        let mut combination_count_vec = Vec::new();
        

        //check each word left in database for what letter pattern they have
        for (_i, word) in self.word_database.iter().enumerate(){
            let mut temp_vec= Vec::new();

            //check word bytes for what index pattern they have
            for w in 0..self.word_length{
                let c = word.as_bytes()[w];

                if c == letter{
                    temp_vec.push(w);
                }
            }

            //Add combinations to combination vector and increment relative combination count
            if !combination_vec.is_empty(){
                combination_vec.push(temp_vec);
                combination_count_vec.push(1);
                
            }else{

                //Add to word count for that specific combination if already in vector
                if combination_vec.contains(&temp_vec){
                    for i in 0..combination_vec.len(){
                        let com = &combination_vec[i];

                        if com.eq(&temp_vec){
                            combination_count_vec[i] += 1;
                        }
                    }
                }else{
                    combination_vec.push(temp_vec);
                    combination_count_vec.push(1);
                }
            }
        }


        return (combination_vec, combination_count_vec);
    }

    /*Remove words that exceed the letter repeat count*/
    fn remove_multiples(&mut self, guess:&str, limit:usize){
        let gc = guess.as_bytes()[0];
        let mut count = 0; let mut remove_vec = Vec::new();

        for (word_index, w) in self.word_database.iter().enumerate(){

            for i in 0..self.word_length{
                let c = w.as_bytes()[i];
                
                if gc == c{
                    count += 1;
                }
            }

            if count > limit{
                remove_vec.push(word_index);
            }
            count = 0;
        }
        for (_i, r) in remove_vec.iter().enumerate(){
            self.word_database.swap_remove(*r);
        }

    }

    fn remove_guess_words(&mut self, list:Vec<String>){
        self.word_database = Vec::new();
        for w in list{
            self.word_database.push(w.to_string());
        }
        
    }

    /*guess cycle */
    pub fn ask_guess(&mut self){

        print!("Enter guess: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let mut answer = line.trim();

        while self.guessed_letters.contains(&answer.to_string()) || answer.len() > 1{
            println!();
            if answer.len() > 1{
                print!("Please enter in only 1 letter: ");
            }else{
                print!("Please enter in something you haven't guessed before: ");
            }
            
            std::io::stdout().flush().unwrap();
            line = String::new();
            std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
            answer = line.trim();
        }

        /*Categorize words here
        * I think I want to categorize them by how many words do and do not have the letter specified
        * make sure to keep running guess list in mind when making these categories
        */
        let (has_guess, list) = self.categorize(answer);
        if has_guess{

            //categorize again by how many and what indices they're most common at
            let (num, plural) = self.categorize_add_guess(answer);

            if plural{
                println!("There are {} copies of '{}' in the word!", num, answer);
            }else{
                println!("There is 1 copy of '{}' in the word!", answer);
            }
            

        }else{
            self.remove_guess_words(list);
            println!("Sorry! There are no {}'s in the word!", answer);
            self.guesses += 1;
        }
        self.add_guessed_letter(answer);
    }
    
}

fn main() {
    print!("Debug or Run?: ");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
    let answer = line.trim();

    if answer.to_lowercase().eq("debug"){

        print!("Debug what? 1:make_word_database, 2:categorize, 3:guess, 4:ask length, 5:random word, 6:guessed correct letter || answer: ");
        std::io::stdout().flush().unwrap();
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok().expect("failed to read line");
        let debug = line2.trim();
        println!();

        let mut h = Hangman{ ..Default::default() };
        h.set_length(6);
        h.make_word_database();
        if debug.eq("1"){
            println!("for 6-letter words");
            println!("{}", h.count_database());

        }else if debug.eq("2"){
            println!("Testing how many 6-letter words after guessing 'u'");
            println!("{}", h.count_database());
            let ( _u, list) = h.categorize("u");
            h.remove_guess_words(list);
            println!("{}", h.count_database());
            println!();
            
            h.make_word_database();
            println!("After guessing 'a'");
            let (_a, list1) = h.categorize("a");
            h.remove_guess_words(list1);
            println!("{}", h.count_database());

        }else if debug.eq("3"){
            h.set_length(4);
            h.make_word_database();
            println!("{}", h.count_database());
            h.ask_guess();
            println!("{}", h.count_database());

        }else if debug.eq("4"){
            println!("{}", h.count_database());
            println!();
            h.ask_length();
            h.make_word_database();
            println!("{}", h.count_database());

        }else if debug.eq("5"){
            println!("Random word from list of 6-letter words: {}", h.choose_random_word());
        }else if debug.eq("6"){
            println!("Testing if word filled after gettings rid of 'u, e, i, a, y, o'");
            h.make_init_word();

            let ( _u, list) = h.categorize("u");
            h.remove_guess_words(list);
            let ( _e, list1) = h.categorize("e");
            h.remove_guess_words(list1);
            let ( _i, list2) = h.categorize("i");
            h.remove_guess_words(list2);
            let ( _a, list3) = h.categorize("a");
            h.remove_guess_words(list3);
            let ( _y, list4) = h.categorize("y");
            h.remove_guess_words(list4);
            let ( _o, list5) = h.categorize("o");
            h.remove_guess_words(list5);
            println!("A word I know {}:",h.choose_random_word());

            h.ask_guess();
            println!("{}",h.word);
            println!();
        }else{
            println!("not a valid testing number");
        }

    }else{
        println!("Welcome to hangman! Hope you enjoy the game~");
        let mut game = Hangman{ ..Default::default()};

        //-------------Initialize the player settings----------------//
        let mut length = game.ask_length();
        while !game.check_length(length){
            println!("I don't know any words with {} letters. Please choose a different length.", length);
            length = game.ask_length();
        }
        let init_length = length as usize;
        game.set_length(init_length);
        println!();

        let mut init_guess = game.ask_initial_guesses();
        while init_guess <= 0{
            println!("Well I mean, I'd already win if you want {} guesses, but it's not fun if you don't try", init_guess);
            init_guess = game.ask_initial_guesses();
        }
        game.set_initial_guesses(init_guess);
        println!();

        let read_word_count_bool = game.ask_show_database_count();
        if read_word_count_bool{
            println!("Gotcha! I'll tell you how many words I could be thinking of after each of your guesses");
        }else{
            println!("Aww, don't wanna see how knowledgeable I am? Alright I won't tell you how many words I could be thinking of after each guess.");
        }
        println!();

        game.make_word_database();
        game.make_init_word();
        println!("Alrighty that's all my pre-screening done, let's start the game!");
        //---------------End of Game initialization------------------//

        println!();
        println!("Note: You only lose a guess if you guess incorrectly");
        while game.get_guesses_left() > 0 && !game.check_word_filled(){
            println!("You have {} guesses left.", game.get_guesses_left());
            println!("Your previous guesses: {:?}", game.guessed_letters);
            println!("The current word {}", game.word);
            game.ask_guess();

            if !game.check_word_filled() && read_word_count_bool{
                println!("I could be thinking one of {} words", game.count_database());
            }
            println!();
        }
        if game.check_word_filled(){
            println!("Congratulations! You guessed the word correctly, that's quite the achievement!");
        }else{
            println!("Guesses depleted: Sorry, but the word I was thinking of was {}", game.choose_random_word());
        }


    }
}
