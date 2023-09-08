

# LinesCoodec 源码分析



### 一、字段详细设计

LinesCodec 有三个字段：

* next_index，是记录下一个值的开始索引。注意是索引值，是从 0 开始，比如输入的是：abc，那么下一个输入的值的索引就是 3
* max_length，最大允许的行长度。
* is_discarding，是否丢弃数据。配合 max_length 使用，如果超过最大长度了，就把数据丢弃。



### 二、方法详细设计

~~~rust
// 初始化方法
fn new() -> LinesCodec{}

// 解码方法
fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, LinesCodecError> {}

// 编码方法
fn encode(&mut self, line: T, buf: &mut BytesMut) -> Result<(), LinesCodecError> {}
~~~



#### 1、初始化方法

初始化方法，初始化 LinesCodec 的所有字段值。



#### 2、解码方法

输入是 BytesMut，一个字节类型的数据结构，因为【网络通信是以字节流的形式传输的】。输出是 String，解码字节流得到字符串。



流程如下：

1. 从输入的字节序列 buf 中读取数据的长度。
2. 获取换行符的索引值。
3. 匹配处理，匹配条件是（是否丢弃，换行符索引）：
   * 如果是丢弃状态，且找到了换行符。就把换行符之前的数据丢弃，然后停止丢弃状态，然后继续下一次匹配。
   * 如果是丢弃状态，但是没有找到换行符。就直接把所有的数据都丢掉，然后继续下一次匹配。
   * 如果不是丢弃状态，但是找到了换行符。就找到了一行数据，然后把这一行数据从 buf 中分离出来，然后返回。【重要】
   * 如果不是丢弃状态，也没有找到换行符。但是 buf 的长度超过了最大长度，就返回错误，然后进入丢弃状态，然后继续下一次匹配。【注意】
   * 如果不是丢弃状态，也没有找到换行符。且 buf 的长度没有超过最大长度，就把下一次读取的索引值设置为当前读取的长度，然后继续下一次匹配。【注意】

【注意】只有在长度超出最大限制的时候，才会进入丢弃状态。



【注意】如果不是丢弃状态，也没有找到换行符，也没有超过长度限制。这是一种特殊情况，也就是由于网络等问题，接收到的数据不完整
* 这时候，需要更新下一次解码的起始位置 next_index，并返回一个表示继续等待更多数据的信号。
* 在异步编程中，返回 Ok(None) 通常用于表示“需要等待更多数据”这一情况，以便异步任务可以暂时挂起并等待更多数据的到来。






b'\n' 表示换行符字符，它的 ASCII 值是 10。
~~~rust
.position(|b| *b == b'\n')
~~~



输入的长度已经超出限制了，但是现在有换行。保留最新数据，丢弃换行之前的数据。

~~~rust
// (true，Some)
buf.advance(offset + self.next_index + 1);
~~~



输入长度已经超出限制了，还没有换行。那就直接丢弃所有数据。

~~~rust
// (true，None)
buf.advance(read_to);
~~~



输入长度没有超出限制，但是有换行。那就找到了一行数据，然后把这一行数据从 buf 中分离出来，然后返回。

【split_to】的作用是提取数据，提取的数据将会在 buf 中消失，索引值也会发生变化。

~~~rust
// (false，Some)

// 1、找到换行符的索引值
let newline_index = offset + self.next_index;
// 2、将 next_index 重置为 0
self.next_index = 0;

// 3、从 buf 中将一行数据分离出来，包括换行符
let line = buf.split_to(newline_index + 1);
// 4、去掉换行符
let line = &line[..line.len() - 1];

// 5、将字节序列转换为字符串
let line = without_carriage_return(line);
let line = utf8(line)?;
~~~



【注意】一些概念：网络通信、字节、二进制、十六进制、计算机数据的基本单位

* 网络通信中的数据是以字节流的形式传输的。
* 但这些字节是以二进制的形式组成的。每个字节都是一个 8 位的二进制数据单元。
* 十进制和十六进制通常用于数据的表示和显示。8 位二进制显示太长了，所以通常用十六进制表示，如 10111010 转换为 BA。
* 字节是计算机中存储和传输数据的基本单位。



| 字节   | 二进制    | 十进制 | ASCII           |
| :----- | :-------- | :----- | :-------------- |
| 1 字节 | 8 位      | 2 位   | 1 个 ASCII 字符 |
| 1 byte | 1111 1111 | FF     | 1 个 ASCII 字符 |

  

#### 3、编码方法

输入是 String，输出是 BytesMut。



流程如下：


1. 分配空间，空间大小是输入字符串的长度 + 1，多出来的 1 是给换行符。
2. 将输入字符串转换为字节序列，然后放入到 buf 中。
3. 在 buf 中添加换行符。





### 三、源码

~~~rust
use crate::codec::decoder::Decoder;
use crate::codec::encoder::Encoder;

use bytes::{Buf, BufMut, BytesMut};
use std::{cmp, fmt, io, str, usize};

/// A simple [`Decoder`] and [`Encoder`] implementation that splits up data into lines.
///
/// [`Decoder`]: crate::codec::Decoder
/// [`Encoder`]: crate::codec::Encoder
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LinesCodec {
    // Stored index of the next index to examine for a `\n` character.
    // This is used to optimize searching.
    // For example, if `decode` was called with `abc`, it would hold `3`,
    // because that is the next index to examine.
    // The next time `decode` is called with `abcde\n`, the method will
    // only look at `de\n` before returning.
    next_index: usize,

    /// The maximum length for a given line. If `usize::MAX`, lines will be
    /// read until a `\n` character is reached.
    max_length: usize,

    /// Are we currently discarding the remainder of a line which was over
    /// the length limit?
    is_discarding: bool,
}

impl LinesCodec {
    /// Returns a `LinesCodec` for splitting up data into lines.
    ///
    /// # Note
    ///
    /// The returned `LinesCodec` will not have an upper bound on the length
    /// of a buffered line. See the documentation for [`new_with_max_length`]
    /// for information on why this could be a potential security risk.
    ///
    /// [`new_with_max_length`]: crate::codec::LinesCodec::new_with_max_length()
    pub fn new() -> LinesCodec {
        LinesCodec {
            next_index: 0,
            max_length: usize::MAX,
            is_discarding: false,
        }
    }

    /// Returns a `LinesCodec` with a maximum line length limit.
    ///
    /// If this is set, calls to `LinesCodec::decode` will return a
    /// [`LinesCodecError`] when a line exceeds the length limit. Subsequent calls
    /// will discard up to `limit` bytes from that line until a newline
    /// character is reached, returning `None` until the line over the limit
    /// has been fully discarded. After that point, calls to `decode` will
    /// function as normal.
    ///
    /// # Note
    ///
    /// Setting a length limit is highly recommended for any `LinesCodec` which
    /// will be exposed to untrusted input. Otherwise, the size of the buffer
    /// that holds the line currently being read is unbounded. An attacker could
    /// exploit this unbounded buffer by sending an unbounded amount of input
    /// without any `\n` characters, causing unbounded memory consumption.
    ///
    /// [`LinesCodecError`]: crate::codec::LinesCodecError
    pub fn new_with_max_length(max_length: usize) -> Self {
        LinesCodec {
            max_length,
            ..LinesCodec::new()
        }
    }

    /// Returns the maximum line length when decoding.
    ///
    /// ```
    /// use std::usize;
    /// use tokio_util::codec::LinesCodec;
    ///
    /// let codec = LinesCodec::new();
    /// assert_eq!(codec.max_length(), usize::MAX);
    /// ```
    /// ```
    /// use tokio_util::codec::LinesCodec;
    ///
    /// let codec = LinesCodec::new_with_max_length(256);
    /// assert_eq!(codec.max_length(), 256);
    /// ```
    pub fn max_length(&self) -> usize {
        self.max_length
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

impl Decoder for LinesCodec {
    type Item = String;
    type Error = LinesCodecError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, LinesCodecError> {
        loop {
            // Determine how far into the buffer we'll search for a newline. If
            // there's no max_length set, we'll read to the end of the buffer.
            let read_to = cmp::min(self.max_length.saturating_add(1), buf.len());

            let newline_offset = buf[self.next_index..read_to]
                .iter()
                .position(|b| *b == b'\n');

            match (self.is_discarding, newline_offset) {
                (true, Some(offset)) => {
                    // If we found a newline, discard up to that offset and
                    // then stop discarding. On the next iteration, we'll try
                    // to read a line normally.
                    buf.advance(offset + self.next_index + 1);
                    self.is_discarding = false;
                    self.next_index = 0;
                }
                (true, None) => {
                    // Otherwise, we didn't find a newline, so we'll discard
                    // everything we read. On the next iteration, we'll continue
                    // discarding up to max_len bytes unless we find a newline.
                    buf.advance(read_to);
                    self.next_index = 0;
                    if buf.is_empty() {
                        return Ok(None);
                    }
                }
                (false, Some(offset)) => {
                    // Found a line!
                    let newline_index = offset + self.next_index;
                    self.next_index = 0;
                    let line = buf.split_to(newline_index + 1);
                    let line = &line[..line.len() - 1];
                    let line = without_carriage_return(line);
                    let line = utf8(line)?;
                    return Ok(Some(line.to_string()));
                }
                (false, None) if buf.len() > self.max_length => {
                    // Reached the maximum length without finding a
                    // newline, return an error and start discarding on the
                    // next call.
                    self.is_discarding = true;
                    return Err(LinesCodecError::MaxLineLengthExceeded);
                }
                (false, None) => {
                    // We didn't find a line or reach the length limit, so the next
                    // call will resume searching at the current offset.
                    self.next_index = read_to;
                    return Ok(None);
                }
            }
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<String>, LinesCodecError> {
        Ok(match self.decode(buf)? {
            Some(frame) => Some(frame),
            None => {
                // No terminating newline - return remaining data, if any
                if buf.is_empty() || buf == &b"\r"[..] {
                    None
                } else {
                    let line = buf.split_to(buf.len());
                    let line = without_carriage_return(&line);
                    let line = utf8(line)?;
                    self.next_index = 0;
                    Some(line.to_string())
                }
            }
        })
    }
}

impl<T> Encoder<T> for LinesCodec
where
    T: AsRef<str>,
{
    type Error = LinesCodecError;

    fn encode(&mut self, line: T, buf: &mut BytesMut) -> Result<(), LinesCodecError> {
        let line = line.as_ref();
        buf.reserve(line.len() + 1);
        buf.put(line.as_bytes());
        buf.put_u8(b'\n');
        Ok(())
    }
}

impl Default for LinesCodec {
    fn default() -> Self {
        Self::new()
    }
}

/// An error occurred while encoding or decoding a line.
#[derive(Debug)]
pub enum LinesCodecError {
    /// The maximum line length was exceeded.
    MaxLineLengthExceeded,
    /// An IO error occurred.
    Io(io::Error),
}

impl fmt::Display for LinesCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinesCodecError::MaxLineLengthExceeded => write!(f, "max line length exceeded"),
            LinesCodecError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for LinesCodecError {
    fn from(e: io::Error) -> LinesCodecError {
        LinesCodecError::Io(e)
    }
}

impl std::error::Error for LinesCodecError {}

~~~



### 四、总结

解码的过程大致：
1. 接收字节流
2. 校验数据包完整性
3. 按照协议解析数据



编码的过程大致：
1. 按照协议拼装数据
2. 输出字节流



【注意】解码的过程必然会遇到的问题：数据包不完整。这可能是因为网络问题，有的数据包到达比较晚。
* 所以需要校验数据包的完整性，如果不完整需要等到数据包完整。
* 在 Netty 中，ByteBuf 提供了 resetReaderIndex 方法重置索引，因为只要读取了索引就会发生变化，但是现在读到的数据不完整，需要重置等下次再读。
* 在 tokio 中，BytesMut 没有提供类似高级功能，所以需要在先缓存已经读到的数据，然后下次读到的往里面追加。



【问题】我怎么知道数据包是完整的？
* 通常来说，数据包与数据包之间是明确了完整性标识的，比如换行符、字段长度等。
* 通常来说，数据的接收是一个【循环】的过程，比如 Netty 是事件循环机制，tokio 是使用 loop 函数。持续地接收数据包。


# 实现一个自定义的 Codec
自定义 HexCodec，实现一个简单的协议，协议格式如下：
~~~bash
报文最少长度14字节：  设备地址（6）+ 信息ID（2）+ 功能编码（1）+ 长度（2）+ 最小数据（1）+ CRC（2）
例子：AA AA AA AA AA + BB BB + CC + DD DD + EE + FF FF

~~~


### 一、需要的 crate
*  crc，进行 crc 校验

### 二、要实现的功能
将字符串编码成16进制
将16进制解码成字符串

~~~rust
// 解码方法
fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<String>, HexCodec> {}

// 编码方法
fn encode(&mut self, line: T, buf: &mut BytesMut) -> Result<(), HexCodecError> {}

~~~

