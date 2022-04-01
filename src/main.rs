use std::{env};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        eprintln!("not enough arguments");
    } else if args[1] == "decode"{
        let input = &args[2];
        println!("{}", ronum_decode(input.to_string()));
    } else if args[1] == "encode"{
        let input: u32 = args[2].trim().parse().expect("not a number");
        println!("{}", ronum_encode(input))
    } else {
        eprintln!("plese select eiether encode or decode");
    }
}
fn ronum_encode(mut num: u32) -> String{
    let mut roman: Vec<&str> = vec![];
    let numarals = [("m",1000), ("d",500), ("c",100), ("l",50), ("x",10), ("v",5), ("i",1)];
    for (numarals,values) in numarals{
        let mut i = num / values;
        println!("{}", i * values);
        num -= i * values;
        while i > 0 {
            roman.push(numarals);
            i -= 1;
        }
    }
    
    let roman = simplify(roman);
    "test".to_string()
}
//fixme
fn simplify(roman: Vec<&str>) -> Vec<&str>{
    roman
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
