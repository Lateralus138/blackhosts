// #[path = "utils/debug.rs"] mod debug;
// use debug::{break_point,type_of,print_str};
use std::any::type_name;
use std::process::exit;
pub fn type_of<T>(_: T) ->
    &'static str {type_name::<T>()}
pub fn break_point(exit_code: u8,exit_message: &str) {
    println!("{}",exit_message);
    exit(exit_code as i32);
}
pub fn print_str(name: &str,value: &str) {
    println!("Variable Name:\n{}\nVariable Value:\n{}",name,value);
}