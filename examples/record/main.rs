use std::{
    collections::VecDeque,
    ffi::c_void,
    mem::{size_of, transmute},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use cidre::{
    av, cf,
    cm::{self, SampleBuffer},
    dispatch,
    os::Status,
    sc::{self, stream::StreamOutput},
    vt::{
        self,
        compression_properties::{keys, profile_level},
        EncodeInfoFlags,
    },
};

use tokio::net::UdpSocket;

const MTU_LEN: usize = 1500;
const UDP_HEADER_LEN: usize = 8;

const HEADER_LEN: usize = std::mem::size_of::<Header>(); // 8; // i32 + u16 + u16
const BODY_LEN: usize = (MTU_LEN - UDP_HEADER_LEN) - HEADER_LEN;
const PACKET_LEN: usize = HEADER_LEN + BODY_LEN;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Header {
    // THINK: we can merge channel_id and kind if kind will be relative small at the end
    kind: MessageKind,
    channel_id: u8, // video or audio steam id (during handshake?)
    msg_no: u16,
    packet_count: u16,
    packet_idx: u16,
}

impl Header {
    #[inline]
    fn write(&self, buf: &mut [u8; PACKET_LEN]) {
        buf[0] = self.kind as _;
        buf[1] = self.channel_id;
        buf[2..4].copy_from_slice(&self.msg_no.to_be_bytes());
        buf[4..6].copy_from_slice(&self.packet_count.to_be_bytes());
        buf[6..8].copy_from_slice(&self.packet_idx.to_be_bytes());
    }

    #[inline]
    fn write_idx(idx: u16, buf: &mut [u8; PACKET_LEN]) {
        buf[6..8].copy_from_slice(&idx.to_be_bytes())
    }

    #[inline]
    fn read_msg_no(buf: &[u8; PACKET_LEN]) -> u16 {
        u16::from_be_bytes([buf[2], buf[3]])
    }

    #[inline]
    fn read_packet_count(buf: &[u8; PACKET_LEN]) -> u16 {
        u16::from_be_bytes([buf[4], buf[5]])
    }

    #[inline]
    fn read_packet_idx(buf: &[u8; PACKET_LEN]) -> u16 {
        u16::from_be_bytes([buf[6], buf[7]])
    }

    #[inline]
    fn read_kind(buf: &[u8; PACKET_LEN]) -> MessageKind {
        unsafe { transmute(buf[0]) }
    }

    #[inline]
    fn read_channel_id(buf: &[u8; PACKET_LEN]) -> u8 {
        buf[1]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum MessageKind {
    VideoNonKey,
    VideoKey,
    File,
    Audio,
    Crtl,
    RepeatPacket,
}

pub enum Cmd {
    Repeat {
        msg_no: u16,
        packet_idx: u16,
    },
    Schedule {
        kind: MessageKind,
        body: cf::Retained<SampleBuffer>,
    },
    Stop,
}

struct Sequencer {
    value: u16,
    overs_count: u64,
}

impl Default for Sequencer {
    fn default() -> Self {
        Self {
            value: 0,
            overs_count: 0,
        }
    }
}

impl Sequencer {
    pub fn next(&mut self) -> u16 {
        let (value, over) = self.value.overflowing_add(1);
        if over {
            self.overs_count += 1;
        }
        self.value = value;
        value
    }
}

pub struct Message {
    header: Header,
    body: cf::Retained<SampleBuffer>,
}

impl Message {
    #[inline]
    pub fn no(&self) -> u16 {
        self.header.msg_no
    }

    #[inline]
    pub fn body(&self) -> &[u8] {
        let data_buffer = self.body.data_buffer().unwrap();
        let data = data_buffer.data_pointer().unwrap();
        data
        //    &self.body
    }
}

#[repr(C)]
struct FameCounter {
    counter: u32,
    session: cf::Retained<vt::CompressionSession>,
}

impl FameCounter {
    pub fn _counter(&self) -> u32 {
        self.counter
    }
}

impl StreamOutput for FameCounter {
    extern "C" fn stream_did_output_sample_buffer_of_type(
        &mut self,
        _stream: &sc::Stream,
        sample_buffer: &cm::SampleBuffer,
        _of_type: sc::OutputType,
    ) {
        self.counter += 1;
        // why without println is not working well?
        // println!("frame {:?}", self.counter);

        let img = sample_buffer.image_buffer();
        if img.is_none() {
            return;
        }
        let img = img.unwrap();
        let pts = sample_buffer.presentation_time_stamp();
        let dur = sample_buffer.duration();

        let mut flags = None;

        let res = self
            .session
            .encode_frame(img, pts, dur, None, std::ptr::null_mut(), &mut flags);
        if res.is_err() {
            println!("err {:?}", res);
        }
    }
}

#[inline]
async fn safe_send_to(
    sock: &UdpSocket,
    dest_addr: SocketAddr,
    buf: &[u8],
) -> Result<usize, std::io::Error> {
    loop {
        let r = sock.send_to(buf, dest_addr).await;
        match r {
            Ok(n) => break Ok(n),
            Err(_e) => {
                // TODO: check actual error
                tokio::time::sleep(Duration::from_millis(10)).await;
                continue;
            }
        }
    }
}

async fn sink_message(sock: &UdpSocket, dest_addr: SocketAddr, msg: Arc<Message>) {
    let body = msg.body();
    let body_len = body.len();

    let mut packet = [0u8; PACKET_LEN];
    msg.header.write(&mut packet);

    let packet_count = msg.header.packet_count as usize;
    // one packet message shortcut (may be not full)
    if packet_count == 1 {
        let packet_len = HEADER_LEN + body_len;
        packet[HEADER_LEN..packet_len].copy_from_slice(&body);
        let _r = safe_send_to(sock, dest_addr, &packet[..packet_len]).await;
        return;
    }

    // first packet (full)
    packet[HEADER_LEN..PACKET_LEN].copy_from_slice(&body[0..BODY_LEN]);
    let _r = safe_send_to(sock, dest_addr, &packet).await;

    // middle packets (full)
    let last_idx = packet_count - 1;
    for packet_idx in 1..last_idx {
        let offset = packet_idx * BODY_LEN;
        Header::write_idx(packet_idx as u16, &mut packet);
        packet[HEADER_LEN..PACKET_LEN].copy_from_slice(&body[offset..offset + BODY_LEN]);
        let _r = safe_send_to(sock, dest_addr, &packet).await;
    }

    // last packet (may be not full)
    let offset = last_idx * BODY_LEN;
    let packet_len = HEADER_LEN + body_len - offset;
    Header::write_idx(last_idx as u16, &mut packet);
    packet[HEADER_LEN..packet_len].copy_from_slice(&body[offset..body_len]);
    let _r = safe_send_to(sock, dest_addr, &packet[..packet_len]).await;
}

async fn transmit_task(
    dest_addr: SocketAddr,
    channel_id: u8,
    rx: flume::Receiver<Cmd>,
) -> Result<(), std::io::Error> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let sock = Arc::new(sock);
    let (msg_tx, msg_rx) = flume::unbounded::<Arc<Message>>();
    let send_task_sock = sock.clone();
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = msg_rx.recv_async().await {
            //println!("! {:?}", msg.no());
            sink_message(&send_task_sock, dest_addr, msg).await;
        }
    });

    let mut buf: VecDeque<Arc<Message>> = VecDeque::default();
    let mut sequencer = Sequencer::default();
    // control loop
    while let Ok(cmd) = rx.recv_async().await {
        match cmd {
            Cmd::Repeat { msg_no, packet_idx } => {
                let msg = buf.iter().find(|m| m.header.msg_no == msg_no);
            }
            Cmd::Schedule { kind, body } => {
                let body_len = body.data_buffer().unwrap().data_len();
                let mut segments = body_len / BODY_LEN;
                if segments * BODY_LEN < body_len {
                    segments += 1;
                }
                let header = Header {
                    kind,
                    channel_id,
                    msg_no: sequencer.next(),
                    packet_count: segments as _,
                    packet_idx: 0,
                };
                let msg = Message { header, body };
                let msg = Arc::new(msg);
                buf.push_back(msg.clone());
                if buf.len() > 3 {
                    buf.pop_front();
                }
                let _r = msg_tx.send_async(msg).await;
            }
            Cmd::Stop => {
                buf.clear();
                break;
            }
        }
    }

    _ = send_task.abort();
    Ok(())
}

pub fn create_sender(addr: SocketAddr, channel_id: u8) -> flume::Sender<Cmd> {
    let (msg_tx, msg_rx) = flume::unbounded();
    let _transmit = tokio::spawn(async move { transmit_task(addr, channel_id, msg_rx).await });
    msg_tx
}

extern "C" fn callback2(
    ctx: *mut c_void,
    _: *mut c_void,
    status: Status,
    _flags: EncodeInfoFlags,
    buffer: Option<&SampleBuffer>,
) {
    // println!("compressed");
    if status.is_err() || buffer.is_none() {
        println!("status {:?}", status);
        return;
    }

    //    sender: flume::Sender<Cmd>,
    let ctx = ctx as *mut cf::Retained<av::AssetWriterInput>;
    let ctx = unsafe { ctx.as_ref().unwrap() };

    let buf = buffer.unwrap();
    let data_buffer = buf.data_buffer().unwrap();
    let data = data_buffer.data_pointer().unwrap();
    assert_eq!(data.len(), data_buffer.data_len());
    println!("{:?}", data.len());

    if ctx.is_ready_for_more_media_data() {
        ctx.append_sample_buffer(buf);
    }
}

extern "C" fn callback(
    ctx: *mut c_void,
    _: *mut c_void,
    status: Status,
    _flags: EncodeInfoFlags,
    buffer: Option<&SampleBuffer>,
) {
    // println!("compressed");
    if status.is_err() || buffer.is_none() {
        println!("status {:?}", status);
        return;
    }

    let ctx = ctx as *mut flume::Sender<Cmd>;
    let ctx = unsafe { ctx.as_ref().unwrap() };

    let buf = buffer.unwrap().retained();
    ctx.send(Cmd::Schedule {
        kind: MessageKind::VideoKey,
        body: buf,
    })
    .unwrap();
    //let data_buffer = buf.data_buffer().unwrap();
    //let data = data_buffer.data_pointer().unwrap();
    //assert_eq!(data.len(), data_buffer.data_len());
    //println!("{:?}", data.len());
}

#[tokio::main]
async fn main() {
    let q = dispatch::Queue::serial_with_autoreleasepool();
    let content = sc::ShareableContent::current().await.expect("content");
    let ref display = content.displays()[0];
    let mut cfg = sc::StreamConfiguration::new();
    cfg.set_minimum_frame_interval(cm::Time::new(1, 30));
    cfg.set_width(display.width() as usize * 2);
    cfg.set_height(display.height() as usize * 2);

    let format = cm::FormatDescription::video(
        cm::VideoCodecType::H264,
        display.width() as i32 * 2,
        display.height() as i32 * 2,
        None,
    )
    .unwrap();

    let writer_input =
        av::AssetWriterInput::with_media_type_and_format_hint(av::MediaType::video(), &format);
    writer_input.set_expects_media_data_in_real_time(true);
    let url = cf::URL::from_str("file:///Users/yury/bla.mp4").unwrap();

    let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    writer.add_input(&writer_input);
    writer.start_writing();
    let start = cm::Time::with_seconds(0.2f64, 10_000);
    let start = cm::Clock::host_time_clock().time().add(&start);
    writer.start_session_at_source_time(start);

    //let input = Box::new(writer_input);
    //let addr = SocketAddr::V4("127.0.0.1:8080".parse().unwrap());
    // let addr = SocketAddr::V4("192.168.135.174:8080".parse().unwrap());
    //let addr = SocketAddr::V4("10.0.1.10:8080".parse().unwrap());
    //let addr = SocketAddr::V4("192.168.135.113:8080".parse().unwrap());
    let addr = SocketAddr::V4("172.20.10.1:8080".parse().unwrap());

    let sender = create_sender(addr, 0);
    let input = Box::new(sender);

    let mut session = vt::CompressionSession::new::<c_void>(
        1920, // display.width() as u32 * 2,
        1080, // display.height() as u32 * 2,
        cm::VideoCodecType::HEVC,
        None,
        None,
        Some(callback),
        Box::into_raw(input) as _,
    )
    .unwrap();

    println!(
        "rendering with {}x{}",
        display.width() * 2,
        display.height() * 2
    );

    let bool_true = cf::Boolean::value_true();
    let bool_false = cf::Boolean::value_false();
    let expected_fr = cf::Number::from_i32(30);
    let frame_delay_count = cf::Number::from_i32(0);
    let max_key_frame_interval = cf::Number::from_i32(30 * 5);
    let rate_limit =
        cf::Array::from_type_refs(&[&cf::Number::from_i32(3_000_000), &cf::Number::from_i32(1)])
            .unwrap();

    let mut props = cf::MutableDictionary::with_capacity(10);
    props.insert(keys::real_time(), bool_true);
    props.insert(keys::allow_frame_reordering(), bool_false);
    props.insert(keys::max_key_frame_interval(), &max_key_frame_interval);
    props.insert(keys::data_rate_limits(), &rate_limit);
    props.insert(
        keys::profile_level(),
        profile_level::hevc::main_auto_level(),
    );
    // props.insert(keys::allow_open_gop(), bool_false);
    // props.insert(keys::h264_entropy_mode(), h264_entropy_mode::cabac());
    props.insert(keys::expected_frame_rate(), &expected_fr);
    props.insert(keys::max_frame_delay_count(), &frame_delay_count);

    session.set_props(&props).unwrap();
    session.prepare().unwrap();

    let windows = cf::ArrayOf::<sc::Window>::new().unwrap();
    let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
    let stream = sc::Stream::new(&filter, &cfg);

    let delegate = FameCounter {
        counter: 0,
        session,
    };
    let d = delegate.delegate();
    let mut error = None;
    stream.add_stream_output(&d, sc::OutputType::Screen, Some(&q), &mut error);
    assert!(error.is_none());
    stream.start().await.expect("started");

    // cf::RunLoop::run
    tokio::time::sleep(Duration::from_secs(1200)).await;

    //    dispatch::Queue::main().async_with(move || {
    _ = stream.stop();

    writer.end_session_at_source_time(cm::Clock::host_time_clock().time());
    writer.finish_writing();
    //  })
}
