fn main() {
    let (address, operation) = match parse_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    println!(
        "Me tengo que conectar con el servidor en {} y enviarle el comando {}",
        address, operation
    );
}

// *NO* recomendamos usar strings para los errores de su programa.
// Recomendamos enums, lo hacemos por temas de tiempo ;)
fn parse_arguments() -> Result<(String, String), &'static str> {
    let mut inputs = std::env::args();

    inputs.next();

    let address = inputs
        .next()
        .ok_or("missing address. Usage: cargo run --bin client -- <address> <operation>")?;
    let operation = inputs
        .next()
        .ok_or("missing operation. Usage: cargo run --bin client -- <address> <operation>")?;

    Ok((address, operation))
}
