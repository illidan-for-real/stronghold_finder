fn main() {

    let mut line = String::new();
    println!("enter eye 1 cords:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    let mut split = line.split(' ');

    let mut args: Vec<&str> = Vec::new();

    let mut data: Vec<i32> = Vec::new();

    for i in split {
        args.push(i);
    }

    for i in args {
        println!("{}", i);
    }

    data.push(args[6].parse::<i32>().unwrap());
    data.push(args[7].parse::<i32>().unwrap());
    data.push(args[8].parse::<i32>().unwrap());
    data.push(args[9].parse::<i32>().unwrap());

}
