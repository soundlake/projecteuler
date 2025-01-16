use measure_time::measure_time;
use rust::get_answer;

fn main() {
    println!("The answer is {}", measure_time!(get_answer()));
}
