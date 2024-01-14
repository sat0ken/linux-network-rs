use std::os::fd::AsRawFd;
use std::str::FromStr;
use nix::sys::socket::{socket, AddressFamily, SockType, SockFlag, SockProtocol, SockaddrIn, connect};
use nix::unistd::{close, read};

// 単純なTCPクライアントの実装
fn main() {
    // ソケットの作成
    let sock = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        SockProtocol::Tcp
    ).expect("create socket err");
    // ソケットの設定
    let addr = SockaddrIn::from_str("127.0.0.1:12345").unwrap();

    // サーバに接続
    connect(sock.as_raw_fd(), &addr).unwrap();

    // サーバからデータを受信
    let mut buf  = [0; 100];
    let size = read(sock.as_raw_fd(), &mut buf).unwrap();

    println!("recv size is {}, recv data is {:?}", size,
             String::from_utf8(Vec::from(buf.get(0..size).unwrap())).unwrap());

    close(sock.as_raw_fd()).unwrap();
}