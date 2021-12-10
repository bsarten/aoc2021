use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    println!("begin");

    let matching_tokens : HashMap<char,char> = HashMap::from([('{', '}'),('<', '>'),('(',')'),('[',']')]);
    let score_map : HashMap<char, u64> = HashMap::from([(')', 1), (']', 2), ('}',3),('>', 4)]);
    let mut line_scores : Vec<u64> = vec![];
    for line in reader.lines() {
        let mut token_error = false;
        let mut token_stack : Vec<char> = vec![];
        for token in line?.chars() {
            if matching_tokens.contains_key(&token){
                token_stack.push(token);
                continue;
            }

            let mut expecting : Option<char> = None;
            if token_stack.last() != None {
                expecting = Some(matching_tokens[token_stack.last().unwrap()]);
            }

            if expecting != None && expecting == Some(token) {
                token_stack.pop();
            } else {
                println!("Expected {}, but found {} instead", expecting.unwrap(), token);
                token_error = true;
                break;
            }
       }

       let mut line_score : u64 = 0;
       while !token_error && !token_stack.is_empty() {
            let incomplete_token = token_stack.pop().unwrap();
            line_score = line_score * 5 + score_map[&matching_tokens[&incomplete_token]];
       }

       if line_score != 0 {
            line_scores.push(line_score);
       }
    }

    line_scores.sort();

    println!("middle is {}", line_scores[line_scores.len() / 2]);

    Ok(())
}