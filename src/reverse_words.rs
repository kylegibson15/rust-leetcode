fn reverse_words(s: String) -> String {
    let split = s.split_whitespace();
    let mut answer: Vec<String> = Vec::new();

    for word in split {
        println!("word: {}", word);
        let rev_word = word.chars().rev().collect::<String>();
        answer.push(rev_word);
    }
    answer.join(" ")
}

pub fn reverse_words_test() {
    let input: String = String::from("Let's take LeetCode contest");
    let before = input.clone();
    let output: String = reverse_words(input);
    println!("before: {}\nafter: {}\n=======\n", before, output);
}
