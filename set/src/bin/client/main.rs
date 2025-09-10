use std::{io::Write, net::{TcpStream, ToSocketAddrs}};

fn main() {
    
    let (address, operation) = match parse_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let string_address = format!("{}", address);
    let mut addrs_iter = string_address.to_socket_addrs().unwrap();    
    if let Ok(mut stream) = TcpStream::connect(addrs_iter.next().unwrap()) {
        stream.write(operation.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Conectado al servidor!");
        } else {
        println!("No se pudo conectar...");
        }


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
