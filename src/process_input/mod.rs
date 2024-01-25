pub fn process(input: &mut impl std::io::BufRead, running: &mut bool) -> Result<(), ()> {
    let mut line = String::new();
    let read = input.read_line(&mut line);
    if let Err(num) = read {
        eprintln!("Error while reading");
        *running = false;
        return Err(());
    }
    println!("{}", line.len());
    Ok(())
}