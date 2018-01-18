#![allow(dead_code)]

pub fn if_statement() {
    let temp = 45;
    if temp > 30 {
        println!("really hot outside");
    } else  if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is ok")
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("it is {}", 
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"}
    );

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"ok"}
    );
}

pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *=2;

        if x == 64 { continue; }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }

}

pub fn for_loop() {
    for x in 1..11 {
        if x == 3 { continue };
        
        if x == 8 { break; }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub fn match_statement() {
    let country_code = 999; // 1 999

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "Unknown",
        _ => "Invalid"
    };

    println!("the country with code {} is {}", country_code, country)
}