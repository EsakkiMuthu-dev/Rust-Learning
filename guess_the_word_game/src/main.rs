const RANDOM_WORDS: [&str; 6] =
    ["MB", "Windy", "Gomes", "Johnny", "Seoul", "Interesting"];

#[derive(Debug,Clone,Default)]
struct GameApp{
    current_word:String,
    right_guess:Vec<char>,
    wrong_guess:Vec<char>
}

enum GUESS {
    RIGHTU,
    THAPPU,
    SOLLIPUTTA
}

impl GameApp{

    fn start(&mut self){
        self.current_word = RANDOM_WORDS[fastrand::usize(..RANDOM_WORDS.len())].to_lowercase();
        self.right_guess.clear();
        self.wrong_guess.clear();
    }

    fn check_guess(&self , guess:char) -> GUESS{
        if self.right_guess.contains(&guess) || self.wrong_guess.contains(&guess)
        {
            return GUESS::SOLLIPUTTA;
        }

        match self.current_word.contains(guess) {
            true => { 
                GUESS::RIGHTU} ,
            false =>  { 
                GUESS::THAPPU} 
        } 
    }
    fn print_results(&self){
        let result = self.current_word
            .chars()
            .map(|ch|{
                if self.right_guess.contains(&ch){
                    ch
                }else{
                    '*'
                }
            })
            .collect::<String>();
        
        println!(" <---- Result ----->");
        println!("{result}");
    }

    fn take_guess(&mut self,guess:String){
        //If len 0 no guess
        // len 1 char guess
        // len more than 1 whole word guess
        match guess.chars().count(){
            0 => {
                println!(" What Are you doing ? , Try Guess something");
            },
            1 => {
                let guess = guess.chars().next().unwrap();
                match self.check_guess(guess){
                    GUESS::SOLLIPUTTA => { println!("solliputta nee already solliputta")},
                    GUESS::RIGHTU => {
                        self.right_guess.push(guess);
                        println!(" Yes Rightuu!! its contain {guess} ! ")
                    },
                    GUESS::THAPPU =>{
                        self.wrong_guess.push(guess);
                        println!(" No , its thappu !! its does not contain {guess} !");
                    }
                }
            },
            _ =>{
                if self.current_word == guess{
                    println!(" Yes its corrent you guessed it !!");
                }else {
                    println!(" No its does not  contain ")
                }
            }
        }

    }

}

fn main() {
    let mut game = GameApp::default();
    game.start();
    loop {
        println!(" Guess the word: ");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        game.take_guess(guess.trim().to_lowercase());
    }
}
