
pub fn error(line: usize, message: String){
    report(line, "".to_string(),message, &mut false);
}

pub fn report(line: usize, error_where:String, message:String,hasError: &mut bool){

    println!("[line {line}] Error {error_where} => {message} ");
    *hasError = true;
}
