
#[warn(unused_imports)]
use async_std::prelude::*;
use async_std::{os::unix::net::{UnixListener, UnixStream}, stream};
use async_std::task;
use async_std::fs;
use std::str;
use std::net::Shutdown;

pub async fn read_stream(stream: &mut UnixStream, size: usize, stop: u8) -> String{
    let mut buff_vector: Vec<Vec<u8>> = vec![];
    loop{
        let mut buffer= vec![0;size];

        match stream.read(&mut buffer[..]).await{
            Ok(n) => {
                println!("Buffer : {:?}, {}", &buffer , &stop);
                if buffer.iter().any(|&x| x==stop ){
                    buff_vector.push(buffer);
                    break;
                }else {
                    buff_vector.push(buffer);
                }
            
            },
            Err(err) => {
                stream.shutdown(Shutdown::Both).unwrap();
                break;

            }

        }
    }
//transformamos a string el vector
let allchar: Vec<u8> = buff_vector
.concat()
.iter()
.filter_map (|&x| if x!=stop {Some(x)}else {None})
.collect();
str::from_utf8(&allchar).unwrap().to_string()


}
