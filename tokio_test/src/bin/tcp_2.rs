use bytes::{Buf, BufMut, BytesMut};
use std::{cmp, fmt, io, str, usize};
use tokio_util::codec::{Decoder, Encoder};

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
    len: String,
    msg_data: String,
    crc: String,
}

impl Decoder for HexCodec {
    type Item = Msg;
    type Error = HexCodecError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Msg>, HexCodecError> {
        loop {
            // 等待更多数据
            if buf.len() < 14 {
                return Ok(None);
            }

            let dev_id_hex = buf.split_to(6);
            let fun_id_hex = buf.split_to(2);
            let code_hex = buf.split_to(1);
            let len_hex = buf.split_to(2);
            let msg_data_hex = buf.split_to(1);
            let crc_hex = buf.split_to(2);

            println!(
                "dev_id_hex={:?}, fun_id_hex={:?}, code_hex={:?}, len_hex={:?}, msg_data_hex={:?}, crc_hex={:?}",
                dev_id_hex, fun_id_hex, code_hex, len_hex, msg_data_hex, crc_hex
            );

            // 使用 format! 宏将字节转换为十六进制字符串
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

            println!(
                "dev_id={:?}, fun_id={:?}, code={:?}, len={:?}, msg_data={:?}, crc={:?}",
                dev_id, fun_id, code, len, msg_data, crc
            );

            let msg = Msg {
                dev_id,
                fun_id,
                code,
                len,
                msg_data,
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

    fn encode(&mut self, msg: Msg, buf: &mut BytesMut) -> Result<(), HexCodecError> {
        // 实现编码逻辑，将 Msg 结构编码成十六进制字符串并附加到 buf 中
        buf.reserve(14);

        let mut vec = vec![];
        let mut dev_id = hex_string_to_bytes(&msg.dev_id).unwrap();
        let mut fun_id = hex_string_to_bytes(&msg.fun_id).unwrap();
        let mut code = hex_string_to_bytes(&msg.code).unwrap();
        let mut len = hex_string_to_bytes(&msg.len).unwrap();
        let mut msg_data = hex_string_to_bytes(&msg.msg_data).unwrap();
        let mut crc = hex_string_to_bytes(&msg.crc).unwrap();
        vec.append(&mut dev_id);
        vec.append(&mut fun_id);
        vec.append(&mut code);
        vec.append(&mut len);
        vec.append(&mut msg_data);
        vec.append(&mut crc);

        buf.extend_from_slice(&vec);

        println!("buf={:?}", buf);

        Ok(())
    }
}

impl Default for HexCodec {
    fn default() -> Self {
        Self::new()
    }
}

/// An error occurred while encoding or decoding a line.
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

fn main() {}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use tokio_util::codec::{Decoder, Encoder};

    use crate::{HexCodec, Msg};

    #[test]
    pub fn test() {
        // 创建 HexCodec 实例，或者根据需要的构造方式创建
        let mut hex_codec = HexCodec::new();

        // dev_id（6 byte）+ fun_id（2 byte）+ code（1 byte）+ len（2 byte）+ msg（1 byte）+ crc（2 byte）
        // AA AA AA AA AA AA + BB BB + CC + DD DD + EE + FF FF
        let msg = Msg {
            dev_id: "AAAAAAAAAAAA".to_string(),
            fun_id: "BBBB".to_string(),
            code: "CC".to_string(),
            len: "DDDD".to_string(),
            msg_data: "EE".to_string(),
            crc: "FFFF".to_string(),
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
