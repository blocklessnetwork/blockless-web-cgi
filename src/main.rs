use bytecodec::DecodeExt;
use httpcodec::{
    HttpVersion, 
    ReasonPhrase, 
    Request, 
    RequestDecoder, 
    Response, 
    StatusCode
};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::{net::TcpListener, os::fd::FromRawFd};



fn handle_http(_req: Request<String>) -> bytecodec::Result<Response<String>> {
    let extensions = blockless_sdk::CGIListExtensions::new();
    let extensions = extensions.expect("CGIListExtensions new error.");
    let command = extensions.command("cgi-web", Vec::new(), Vec::new());
    let result = match command {
        Ok(mut command) => command.exec_command().expect("execute the cgi error"),
        Err(e) => format!("commond found error. {e:?}"),
    };
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        result,
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
    let rs = blockless_sdk::create_tcp_bind_socket("0.0.0.0:8080").unwrap();
    let listener = unsafe { TcpListener::from_raw_fd(rs as _) };
    loop {
        let rs = listener.accept().unwrap();
        let _ = handle_client(rs.0);
    }
}
