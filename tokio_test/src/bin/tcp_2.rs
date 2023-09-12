use bytes::{Buf, BufMut, BytesMut};
use crc::{Crc, CRC_16_IBM_SDLC};
use futures_util::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt};
use std::{fmt, io, str, usize};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::codec::{Decoder, Encoder, Framed};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HexCodec {
    next_index: usize,
    max_length: usize,
    is_discarding: bool,
}

impl HexCodec {
    pub fn new() -> HexCodec {
        HexCodec {
            next_index: 0,
            max_length: usize::MAX,
            is_discarding: false,
        }
    }
}

fn utf8(buf: &[u8]) -> Result<&str, io::Error> {
    str::from_utf8(buf)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to decode input as UTF8"))
}

fn without_carriage_return(s: &[u8]) -> &[u8] {
    if let Some(&b'\r') = s.last() {
        &s[..s.len() - 1]
    } else {
        s
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Msg {
    dev_id: String,
    fun_id: String,
    code: String,
    len: u16,
    msg_data: MsgData,
    crc: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MsgData {
    mode: u8,
    op_code: u8,
}

impl Decoder for HexCodec {
    type Item = Msg;
    type Error = HexCodecError;

    // 将字节解码为16进制字符串
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Msg>, HexCodecError> {
        loop {
            // 去除换行符
            while buf.len() > 0 && (buf[0] == b'\n' || buf[0] == b'\r') {
                buf.advance(1);
            }

            // 等待更多数据
            if buf.len() < 14 {
                return Ok(None);
            }

            // 取出缓冲区数据
            let dev_id_hex = buf.split_to(6);
            let fun_id_hex = buf.split_to(2);
            let code_hex = buf.split_to(1);
            let len_hex = buf.split_to(2);
            let msg_data_hex = buf.split_to(2);
            let crc_hex = buf.split_to(2);

            println!(
                "dev_id_hex={:?}, fun_id_hex={:?}, code_hex={:?}, len_hex={:?}, msg_data_hex={:?}, crc_hex={:?}",
                dev_id_hex, fun_id_hex, code_hex, len_hex, msg_data_hex, crc_hex
            );

            // 使用 format! 宏将字节转换为十六进制字符串
            // 如："[0x01 0x02]"=>"0102"
            // 02表示转换后的16进制字符串最少是2位，不足补0，即一个字节
            // X表示转换为大写的16进制
            let dev_id = dev_id_hex
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join("");
            let fun_id = fun_id_hex
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join("");
            let code = code_hex
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join("");
            let len = len_hex
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join("");
            let msg_data = msg_data_hex
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join("");
            let crc = crc_hex
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join("");

            // 进行crc校验
            let msg_less_crc = format!(
                "{}{}{}{}{}{}",
                dev_id,
                fun_id,
                code,
                len,
                msg_data.get(0..2).unwrap(),
                msg_data.get(2..4).unwrap()
            );

            let crc_check = Crc::<u16>::new(&CRC_16_IBM_SDLC);
            let crc_check = crc_check.checksum(msg_less_crc.as_bytes());
            let crc_check = format!("{:04X}", crc_check);
            println!(
                "msg_less_crc={:?}, crc={:?}, crc_check={:?}",
                msg_less_crc, crc, crc_check
            );
            assert_eq!(crc, crc_check);

            // msg_data的前面一个字节是mode，后面一个字节是op_code
            let mode = msg_data.get(0..2).unwrap();
            let op_code = msg_data.get(2..4).unwrap();
            let msg_data = MsgData {
                mode: u8::from_str_radix(mode, 16).unwrap(),
                op_code: u8::from_str_radix(op_code, 16).unwrap(),
            };

            println!(
                "dev_id={:?}, fun_id={:?}, code={:?}, len={:?}, msg_data={:?}, crc={:?}",
                dev_id, fun_id, code, len, msg_data, crc
            );

            let msg = Msg {
                dev_id,
                fun_id,
                code,
                len: u16::from_str_radix(&len, 16).unwrap(),
                msg_data: msg_data,
                crc,
            };
            println!("{:?}", msg);

            return Ok(Some(msg));
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "bytes remaining on stream").into())
                }
            }
        }
    }

    fn framed<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Sized>(
        self,
        io: T,
    ) -> tokio_util::codec::Framed<T, Self>
    where
        Self: Sized,
    {
        tokio_util::codec::Framed::new(io, self)
    }
}

impl Encoder<Msg> for HexCodec {
    type Error = HexCodecError;

    // 将msg中16进制的数据编码为字节。两种类型的数据：1)数字 2)字符串
    // 如：数字中："10"=>[0x0A], 字符串中："AA"=>[0xAA]
    fn encode(&mut self, msg: Msg, buf: &mut BytesMut) -> Result<(), HexCodecError> {
        // 数字类型
        let mode = format!("{:02X}", msg.msg_data.mode);
        let op_code = format!("{:02X}", msg.msg_data.op_code);
        let data = mode + &op_code;
        let len = format!("{:04X}", data.len());

        // 字符串类型
        let dev_id = msg.dev_id;
        let fun_id = msg.fun_id;
        let code = msg.code;
        let crc = msg.crc;

        // 获得所有的16进制字符串
        let msg = format!("{}{}{}{}{}{}", dev_id, fun_id, code, len, data, crc);
        println!("msg={:?}", msg);

        // 使用hex_string_to_bytes，将16进制字符串传字节。
        // 而不是使用as_bytes，因为as_bytes是按照普通字符串转。
        buf.reserve(16);
        buf.put(hex_string_to_bytes(&msg).unwrap().as_slice()); // as_slice 可以将具有切片语义的数据结构转换为切片，如：Vec<u8>=>[u8]

        println!("buf={:?}", buf);

        Ok(())
    }
}

impl Default for HexCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum HexCodecError {
    Io(io::Error),
}

impl fmt::Display for HexCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexCodecError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for HexCodecError {
    fn from(e: io::Error) -> HexCodecError {
        HexCodecError::Io(e)
    }
}

impl std::error::Error for HexCodecError {}

/*
 * 将十六进制字符串转换为为字节数组
*/
fn hex_string_to_bytes(hex_string: &str) -> Option<Vec<u8>> {
    // 去除字符串前缀 "0x"（如果存在）
    let hex_string = hex_string.trim_start_matches("0x");

    // 检查十六进制字符串的长度是否为偶数
    if hex_string.len() % 2 != 0 {
        return None; // 长度不合法，无法解析
    }

    // 将每两个字符解析为一个字节
    let mut bytes = Vec::new();
    let mut i = 0;
    while i < hex_string.len() {
        let byte = match u8::from_str_radix(&hex_string[i..i + 2], 16) {
            Ok(b) => b,
            Err(_) => return None, // 解析失败，无法解析
        };
        bytes.push(byte);
        i += 2;
    }

    Some(bytes)
}

// 为了方便，将类型别名定义为LineFramedStream和LineFramedSink
type HexFramedStream = SplitStream<Framed<TcpStream, HexCodec>>;
type HexFramedSink = SplitSink<Framed<TcpStream, HexCodec>, Msg>;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    while let Ok((client_stream, _client_addr)) = server.accept().await {
        // 每接入一个客户端的连接请求，都分配一个子任务，
        // 如果客户端的并发数量不大，可为每个客户端都分配一个thread
        tokio::spawn(async move {
            process_client(client_stream).await;
        });
    }
}

async fn process_client(client_stream: TcpStream) {
    // 将TcpStream转换为Framed
    let framed = Framed::new(client_stream, HexCodec::new());

    // 将Framed分离，可得到独立的读写端
    let (frame_writer, frame_reader) = framed.split::<Msg>();

    // 当Reader从客户端读取到数据后，发送到通道中，
    // 另一个异步任务读取该通道，从通道中读取到数据后，将内容按行写给客户端
    let (msg_tx, msg_rx) = mpsc::channel::<Msg>(100);

    // 负责读客户端的异步子任务
    let mut read_task = tokio::spawn(async move {
        read_from_client(frame_reader, msg_tx).await;
    });

    // 负责向客户端写行数据的异步子任务
    let mut write_task = tokio::spawn(async move {
        write_to_client(frame_writer, msg_rx).await;
    });

    // 无论是读任务还是写任务的终止，另一个任务都将没有继续存在的意义，因此都将另一个任务也终止
    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        eprintln!("read_task/write_task terminated");
        read_task.abort();
        write_task.abort();
    };
}

async fn read_from_client(mut reader: HexFramedStream, msg_tx: mpsc::Sender<Msg>) {
    loop {
        match reader.next().await {
            None => {
                println!("client closed");
                break;
            }
            Some(Err(e)) => {
                eprintln!("read from client error: {}", e);
                break;
            }
            Some(Ok(str)) => {
                println!("read from client. content: {:?}", str);
                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if msg_tx.send(str).await.is_err() {
                    eprintln!("receiver closed");
                }
            }
        }
    }
}

async fn write_to_client(mut writer: HexFramedSink, mut msg_rx: mpsc::Receiver<Msg>) {
    while let Some(str) = msg_rx.recv().await {
        if writer.send(str).await.is_err() {
            eprintln!("write to client failed");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HexCodec, Msg, MsgData};
    use bytes::BytesMut;
    use crc::{Crc, CRC_16_IBM_SDLC};
    use tokio_util::codec::{Decoder, Encoder};

    #[test]
    pub fn test() {
        // 创建 HexCodec 实例，或者根据需要的构造方式创建
        let mut hex_codec = HexCodec::new();

        // dev_id（6 byte）+ fun_id（2 byte）+ code（1 byte）+ len（2 byte）+ msg（1 byte）+ crc（2 byte）
        // AA AA AA AA AA AA + BB BB + CC + DD DD + EE + FF FF

        let msg_data = MsgData {
            mode: 1,
            op_code: 1,
        };

        let dev_id = "AAAAAAAAAAAA".to_string();
        let fun_id = "BBBB".to_string();
        let code = "CC".to_string();
        let len = format!("{:04x}", 4);
        let mode = format!("{:02x}", msg_data.mode);
        let op_code = format!("{:02x}", msg_data.op_code);
        let msg_less_crc = format!("{}{}{}{}{}{}", dev_id, fun_id, code, len, mode, op_code);

        let crc = Crc::<u16>::new(&CRC_16_IBM_SDLC);
        let crc = crc.checksum(msg_less_crc.as_bytes());
        let crc = format!("{:04X}", crc);
        println!("msg_less_crc={}, crc={:?}", msg_less_crc, crc);

        let msg = Msg {
            dev_id: "AAAAAAAAAAAA".to_string(),
            fun_id: "BBBB".to_string(),
            code: "CC".to_string(),
            len: 4,
            msg_data: msg_data,
            crc: crc,
        };

        let mut encoded_buffer = BytesMut::new();
        let _ = hex_codec.encode(msg, &mut encoded_buffer).unwrap();

        let msg = hex_codec.decode(&mut encoded_buffer);
        match msg {
            Ok(Some(msg)) => println!("{:?}", msg),
            Ok(None) => println!("No more messages"),
            Err(_) => println!("Decode error"),
        }
    }
}
