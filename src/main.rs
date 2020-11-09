use regex::Regex;
use std::env;

fn main() {
  let mut args: Vec<String> = env::args().collect();
  let mut send: String = String::from("");
  args.remove(0);
  for x in args {
    let y: &str = &format!(" {}", x);
    send.push_str(&y);
  }
  let send2 = send.trim_start();
  println!("new string with vowels removed: \"{}\"", replace_vowels(send2));
}

fn replace_vowels(s: &str) -> String {
  let re = Regex::new(r"[aeiuo]").unwrap();
  re.replace_all(s, "").to_string()
}
