use tokio::{io, net::TcpListener, prelude::*};

fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            // Get readable and writeable parts of the socket stream
            // splits into Send and Sync objects
            let (reader, writer) = socket.split();

            // Copy bytes
            let amount = io::copy(reader, writer);

            // this function returns a future that tokio can use
            let msg = amount.then(|result| {
                match result {
                    Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });

            // Here we actually "use" the future, spawning a task via tokio
            //
            // This makes connection handling happen concurrently
            tokio::spawn(msg);
            Ok(())
        })
        .map_err(|err| {
            // Handle error with STDOUT
            println!("accept error = {:?}", err);
        });

    println!("server running on {}", addr);

    // Start server
    tokio::run(server);
}
