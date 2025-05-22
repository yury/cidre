#[doc(alias = "compression_algorithm")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Algorithm(pub i32);

/// Commonly-available algorithms
impl Algorithm {
    pub const LZ4: Self = Self(0x100);
    pub const ZLIB: Self = Self(0x205);
    pub const LZMA: Self = Self(0x306);
    pub const LZ4_RAW: Self = Self(0x101);
    pub const BROTLI: Self = Self(0xB02);
}

/// Apple-specific algorithms
impl Algorithm {
    pub const LZFSE: Self = Self(0x801);
    pub const LZBITMAP: Self = Self(0x702);
}

#[doc(alias = "compression_stream_operation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum StreamOp {
    Encode,
    Decode,
}

#[doc(alias = "compression_status")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum Status {
    Ok = 0,
    Err = -1,
    End = 1,
}

#[inline]
pub fn encode_buf(dst: &mut [u8], src: &[u8], algorithm: Algorithm) -> usize {
    unsafe {
        compression_encode_buffer(
            dst.as_mut_ptr(),
            dst.len(),
            src.as_ptr(),
            src.len(),
            std::ptr::null_mut(),
            algorithm,
        )
    }
}

#[inline]
pub fn decode_buf(dst: &mut [u8], src: &[u8], algorithm: Algorithm) -> usize {
    unsafe {
        compression_decode_buffer(
            dst.as_mut_ptr(),
            dst.len(),
            src.as_ptr(),
            src.len(),
            std::ptr::null_mut(),
            algorithm,
        )
    }
}

const COMPRESSION_STREAM_FINALIZE: i32 = 0x0001;

pub fn stream(op: StreamOp, src: &[u8], algorithm: Algorithm) -> Result<Vec<u8>, Status> {
    let size = if op == StreamOp::Encode {
        src.len() / 4
    } else {
        src.len() * 4
    };
    let mut dst = vec![0u8; size.max(1)];
    let mut stream = Stream {
        dst_ptr: dst.as_mut_ptr(),
        dst_size: dst.len(),
        src_ptr: src.as_ptr(),
        src_size: src.len(),
        state: std::ptr::null_mut(),
    };

    unsafe {
        let res = compression_stream_init(&mut stream, op, algorithm);
        if res != Status::Ok {
            return Err(res);
        }
        loop {
            let res = compression_stream_process(&mut stream, COMPRESSION_STREAM_FINALIZE);
            if res == Status::Ok {
                let len = dst.len();
                dst.resize((dst.len() * 2).max(512), 0u8);
                stream.dst_ptr = dst.as_mut_ptr().add(len);
                stream.dst_size = dst.len() - len;
                continue;
            }
            let r = compression_stream_destroy(&mut stream);
            if r != Status::Ok {
                break Err(r);
            }
            break if res == Status::End {
                dst.truncate(dst.len() - stream.dst_size);
                Ok(dst)
            } else {
                Err(res)
            };
        }
    }
}

#[link(name = "compression", kind = "dylib")]
unsafe extern "C" {
    #[link_name = "compression_encode_scratch_buffer_size"]
    pub safe fn encode_scratch_buf_len(algorithm: Algorithm) -> usize;

    fn compression_encode_buffer(
        dst_buf: *mut u8,
        dst_size: usize,
        src_buf: *const u8,
        src_size: usize,
        scratch_buf: *mut u8,
        algorithm: Algorithm,
    ) -> usize;

    #[link_name = "compression_decode_scratch_buffer_size"]
    pub safe fn decode_scratch_buf_len(algorithm: Algorithm) -> usize;

    fn compression_decode_buffer(
        dst_buf: *mut u8,
        dst_size: usize,
        src_buf: *const u8,
        src_size: usize,
        scratch_buf: *mut u8,
        algorithm: Algorithm,
    ) -> usize;

    fn compression_stream_init(
        stream: *mut Stream,
        operation: StreamOp,
        algorithm: Algorithm,
    ) -> Status;

    fn compression_stream_process(stream: *mut Stream, flags: i32) -> Status;

    fn compression_stream_destroy(stream: *mut Stream) -> Status;
}

#[repr(C)]
struct Stream {
    dst_ptr: *mut u8,
    dst_size: usize,
    src_ptr: *const u8,
    src_size: usize,
    state: *mut std::ffi::c_void,
}

#[cfg(test)]
mod tests {
    use crate::compression::*;
    #[test]
    fn basics() {
        let len = encode_scratch_buf_len(Algorithm::ZLIB);
        assert_eq!(270336, len);

        let len = encode_scratch_buf_len(Algorithm(10));
        assert_eq!(0, len);

        let mut encoded_buf = vec![0u8; 100];

        let data = "hello compression!!!!";

        let size = encode_buf(&mut encoded_buf, data.as_bytes(), Algorithm::LZMA);
        assert_eq!(size, 72);

        let mut decoded_buf = vec![0u8; 100];
        let size = decode_buf(&mut decoded_buf, &encoded_buf[..size], Algorithm::LZMA);

        assert_eq!(size, data.len());
        assert_eq!(data.as_bytes(), &decoded_buf[..size]);

        let decoded_buf = stream(StreamOp::Decode, &encoded_buf, Algorithm::LZMA).unwrap();
        assert_eq!(&decoded_buf, data.as_bytes());

        let encoded_buf = stream(StreamOp::Encode, data.as_bytes(), Algorithm::LZMA).unwrap();
        assert_eq!(encoded_buf.len(), 72);

        let decoded_buf = stream(StreamOp::Decode, &encoded_buf, Algorithm::LZMA).unwrap();
        assert_eq!(&decoded_buf, data.as_bytes());
    }
}
