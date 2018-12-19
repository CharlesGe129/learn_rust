struct Stack {
    list: Vec<String>,
}

static LEFT_CHARS: &'static str = "({[";

impl Stack {
    fn new() -> Stack {
        Stack {
            list: Vec::new()
        }
    }

    fn push(&mut self, s: String) -> bool {
        if LEFT_CHARS.contains(&s) {
            self.list.push(if s.len() == 1 {s} else {String::new()});
            return true;
        } else {
            let top_string = self.pop();
            let top = top_string.as_str();
            match s.as_str() {
                ")" => return top == "(",
                "}" => return top == "{",
                "]" => return top == "[",
                _ => return false,
            }
        }
    }

    fn pop(&mut self) -> String {
        if self.list.is_empty() {
            return String::new()
        }
        let len = self.list.len();
        let rs = self.list[len-1].to_string();
        self.list = self.list[..len-1].to_vec();
        return rs;
    }

    fn print(&self) {
        println!("Current stack size: {}", &self.list.len());
        for each in &self.list {
            print!("{}", each);
        }
        println!();
    }
}

fn verify_closable_test() {
    println!("{}", verify_closable("[]{}"));
    println!("{}", verify_closable("[({})]"));
    println!("{}", verify_closable("[({})][]{()}"));
    println!("{}", verify_closable("([)]"));
    println!("{}", verify_closable("([)]"));
    println!("{}", verify_closable("([)"));
    println!("{}", verify_closable("[({})][(]{()}"));
    println!("{}", verify_closable("("));
    println!("{}", verify_closable("]"));
}

fn verify_closable(input_str: &str) -> bool {
    let input = String::from(input_str);
    let mut stack = Stack::new();

    for each in input.chars() {
        if !stack.push(each.to_string()) {
            return false;
        }
    }
    return stack.list.len() == 0;
}