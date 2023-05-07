use async_std::prelude::*;
use async_std::os::unix::net::UnixStream;
use std::net::Shutdown;
use async_std::io;
use std::time::Duration;
mod read;
use read::read_stream;

#[async_std::main]
async fn main() {
    let socket_path="/tmp/rust.socket";
    let mut stream = UnixStream::connect(socket_path).await.unwrap();
    let address = stream.local_addr().unwrap();
    print!("Local Unix Socket {:?}", address);
    let per_address = stream.peer_addr().unwrap();
    print!("Remote Unix Socket {:?}", per_address);
 
    //Lectura del string 
 //   let mut response =String::new();
 //   stream.read_to_string(&mut response  ).await.unwrap();
 //   println!("Mensaje desde server : {}", response);
   
   //lectura del stream
    let stdin = io::stdin();
    let mut line = String::new();
    const usize:usize = 10;
    let stop:u8 = b'\0';



    loop {
        let msg = read_stream(&mut stream, usize, stop).await;

        println!("Recibido desde el servidor {:?}", msg);


        match stdin.read_line(&mut line).await{
            Ok(size) => println!("Recibido input{}", size),
            Err(err)=> eprintln!("Error input")
        };

        println!("Enviando a servidor {}", line);

        match stream.write(line.as_bytes()).await{
            Ok(s) => println!("Enviando {} bytes", s),
            Err(err)=> eprintln!("Error al enviar")
        };
        //confirma que se envie todo 
        stream.flush();

        if &line == "END"{
            println!("Cerrando cliente ...");
            break;
        }

    }

   // 
    stream.shutdown(Shutdown::Both).unwrap();

}