pub fn error(line: i32, message: String){
    report(line, "".to_string(),message, &mut false);
}

pub fn report(line: i32, error_where:String, message:String,hasError: &mut bool){

    println!("[line {line}] Error {error_where} => {message} ");
    *hasError = true;
}
