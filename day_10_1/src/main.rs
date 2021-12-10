use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut score = 0;

    let matching_tokens : HashMap<char,char> = HashMap::from([('{', '}'),('<', '>'),('(',')'),('[',']')]);
    let score_map : HashMap<char, u32> = HashMap::from([(')', 3), (']', 57), ('}',1197),('>', 25137)]);
    let mut token_stack : Vec<char> = vec![];
    for line in reader.lines() {
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
                score += score_map[&token];
                break;
            }
       }
    }

    println!("Score is {}", score);

    Ok(())
}