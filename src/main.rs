use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

        println!("{}", ronum_decode(contents));
}
fn ronum_decode(num: String) -> u32{
    let num = num.to_lowercase();
    let num = num.split("");
    let num: Vec<&str> = num.collect();
    let mut ints: Vec<u32> = vec![];
    for c in num{
        match c{
            "i"=> ints.push(1),
            "v" => ints.push(5),
            "x"=> ints.push(10),
            "l"=> ints.push(50),
            "c" => ints.push(100),
            "d"=> ints.push(500),
            "m"=> ints.push(1000),
            _ => continue
        }
    }
    let mut result: Vec<u32> = vec![];
    let mut i = 0;
    while i < ints.len(){
        if i != ints.len() - 1 && ints[i] < ints[i + 1]{
            result.push(ints[i + 1] - ints[i]);
            i+=1;
        } else {
            result.push(ints[i]);
        }
        i += 1;
    }
    let result: u32 = result.iter().sum();
    result
}