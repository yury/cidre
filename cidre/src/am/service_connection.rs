use std::os::unix::prelude::{FromRawFd, RawFd};

use tokio::io::Interest;

pub use crate::am::device::base::ServiceConnection;

#[derive(Debug)]
pub struct InvalidSocketError;

#[derive(Debug)]
pub struct SendError;

#[derive(Debug)]
pub struct RecvError;

impl ServiceConnection {
    pub fn socket(&self) -> Result<RawFd, InvalidSocketError> {
        unsafe {
            let res = AMDServiceConnectionGetSocket(self);
            if res == -1 {
                Err(InvalidSocketError)
            } else {
                Ok(res)
            }
        }
    }

    #[inline]
    pub fn send(&self, buf: &[u8]) -> Result<usize, SendError> {
        unsafe {
            let res = AMDServiceConnectionSend(self, buf.as_ptr(), buf.len());
            if res >= 0 {
                Ok(res as _)
            } else {
                println!("send: {0}", res);
                Err(SendError)
            }
        }
    }

    #[inline]
    pub fn recv(&self, buf: &mut [u8]) -> Result<usize, RecvError> {
        unsafe {
            let res = AMDServiceConnectionReceive(self, buf.as_mut_ptr(), buf.len());
            if res >= 0 {
                Ok(res as _)
            } else {
                eprintln!("recv: {0}", res);
                Err(RecvError)
            }
        }
    }

    pub async fn http_proxy(&self) -> std::io::Result<u16> {
        use tokio::io::AsyncWriteExt;
        use tokio::net::{TcpListener, TcpStream};

        let std_stream = unsafe { std::net::TcpStream::from_raw_fd(self.socket().unwrap()) };
        std_stream.set_nonblocking(true)?;
        // std_stream.set_nodelay(true)?;
        // std::mem::forget(std_stream);
        // // std_stream.set_nodelay(true)?;
        let device = TcpStream::from_std(std_stream)?;
        // unsafe {
        //     extern "C" fn cb(s: &cf::Socket,
        //         cb_type: cf::SocketCallBackType,
        //         address: &cf::Data,
        //         data: *const u8,
        //         info: *mut c_void) {
        //             println!("??????????")
        //     }
        //     let sock = cf::Socket::create_with_native(None, self.socket().unwrap(), cf::SocketCallBackType::READ, cb, None).unwrap();
        //     let source = sock.create_runloop_source(None, 0).unwrap();
        //     cf::RunLoop::main().add_source(&source, cf::RunLoopMode::common())

        // }

        // let n = self.send(&[0, 0, 0, 0]).unwrap();
        // println!("post send! {}", n);

        //let listener = TcpListener::bind("127.0.0.1:0").await?;
        let listener = TcpListener::bind("127.0.0.1:8088").await?;
        let addr = listener.local_addr()?;
        println!("listening: {}", addr);

        let connection = self.retained();
        tokio::spawn(async move {
            let mut socket = match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("new client: {:?}", addr);
                    socket
                }
                Err(e) => {
                    println!("couldn't get client: {:?}", e);
                    return Err(e);
                }
            };

            let mut buf = vec![0; 0x1000];
            loop {
                let ready = socket
                    .ready(Interest::WRITABLE | Interest::READABLE)
                    .await?;
                if ready.is_readable() {
                    device.writable().await?;
                    match socket.try_read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let send = connection.send(&buf[0..n]).unwrap();
                            println!("send {} {}", send, n);
                        }
                        Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => Err(e)?,
                    }
                }

                if ready.is_writable() {
                    println!("pre readable");
                    device.readable().await?;
                    println!("post readable");
                    match connection.recv(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            println!("recv {}", n);
                            socket.write_all(&buf[0..n]).await?;
                        }
                        Err(e) => {
                            println!("err {:?}", e);
                            continue;
                        }
                    }
                }
            }

            std::io::Result::Ok(())
        });
        Ok(addr.port())
    }
}

unsafe extern "C" {
    fn AMDServiceConnectionGetSocket(connection: &ServiceConnection) -> RawFd;

    fn AMDServiceConnectionSend(
        connection: &ServiceConnection,
        buffer: *const u8,
        length: usize,
    ) -> i32;

    fn AMDServiceConnectionReceive(
        connection: &ServiceConnection,
        buffer: *mut u8,
        length: usize,
    ) -> i32;
}
