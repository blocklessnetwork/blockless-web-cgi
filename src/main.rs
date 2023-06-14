use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::{net::TcpListener, os::fd::FromRawFd};

#[link(wasm_import_module = "blockless_socket")]
extern "C" {
    #[link_name = "create_tcp_bind_socket"]
    fn create_tcp_bind_socket_native(addr: *const u8, addr_len: u32, fd: *mut u32) -> u32;
}

#[derive(Debug)]
enum SocketErrorKind {
    ConnectRefused,
    ParameterError,
    ConnectionReset,
    AddressInUse,
}

fn create_tcp_bind_socket(addr: &str) -> Result<u32, SocketErrorKind> {
    unsafe {
        let addr_ptr = addr.as_ptr();
        let mut fd: u32 = 0;
        let rs = create_tcp_bind_socket_native(addr_ptr, addr.len() as _, (&mut fd) as *mut u32);
        if rs == 0 {
            return Ok(fd);
        }
        Err(match rs {
            1 => SocketErrorKind::ConnectRefused,
            2 => SocketErrorKind::ParameterError,
            3 => SocketErrorKind::ConnectionReset,
            4 => SocketErrorKind::AddressInUse,
            _ => unreachable!("unreach."),
        })
    }
}

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        format!("echo: {}", req.body()),
    ))
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e.to_string());
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() {
    let rs = create_tcp_bind_socket("0.0.0.0:8080").unwrap();
    let listener = unsafe { TcpListener::from_raw_fd(rs as _) };
    loop {
        let rs = listener.accept().unwrap();
        handle_client(rs.0);
    }
}
