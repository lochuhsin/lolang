pub fn error(line: usize, msg: &str) {
    report(line, String::new(), msg)
}

pub fn report(line: usize, wh: String, msg: &str) {
    println!("line[ {} ] Error {} : {}", line, wh, msg);
}

pub fn runtime_error(line: usize, msg: &str) {
    println!("line[ {} ] Runtime Error : {}", line, msg)
}
