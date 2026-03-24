pub fn show_me() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        dbg!(i);
    }
    println!("Length: {}", collatz_length(32))
}

fn collatz_length(mut n: i32) -> u32 {
    // todo!("Implement this");
    let mut len = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}
