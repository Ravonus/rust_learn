fn if_statement() {
    let temp = 32;

    if temp > 30 {
        println!("Puter is hot.")
    } else if temp < 10 {
        println!("Computer is chillllllling")
    } else {
        println!("Computer is normal temps")
    }

    let cpu = if temp > 20 { "sizzled" } else { "coldizzled" };
    println!("cup is {}", cpu);

    //or even do it within print line if its small enough

    println!(
        "Cpu is {}",
        if temp > 20 {
            "sizzled"
        } else if temp < 10 {
            "coldizzled"
        } else {
            "Normalz"
        }
    );

    println!(
        "CPU IS {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}

fn main() {
    if_statement();
}
