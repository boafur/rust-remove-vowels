use regex::Regex;
use std::env::args;

fn main() {
  let mut args: Vec<String> = args().collect();
  let mut send: String = String::new();
  args.remove(0);
  if args.len() <= 0 {
    println!("please enter a string:");
    send = text_io::read!("{}\n");
  } else if args.len() > 0 {
    for x in args {
      let y: &str = &format!(" {}", x);
      send.push_str(&y);
    }
    send = send.trim_start().to_string();
  }
  // println!("{}", send);
  println!("new string with vowels removed: \"{0}\"", replace_vowels(&send));
}

fn replace_vowels(s: &str) -> String {
  let re = Regex::new(r"[aeiuo]").unwrap();
  return re.replace_all(s, "").to_string();
}
