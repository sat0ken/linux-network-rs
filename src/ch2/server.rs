use std::os::fd::AsRawFd;
use std::str::FromStr;
use nix::sys::socket::{socket, AddressFamily, SockType, SockFlag, SockProtocol, bind, SockaddrIn, listen, accept};
use nix::unistd::{close, write};

// 単純なTCPサーバの実装
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
    bind(sock.as_raw_fd(), &addr).unwrap();

    // 接続要求を待てる状態にする
    listen(&sock, 5).expect("listen err");

    // TCP clientからの接続要求を受け付ける
    let c_sock = accept(sock.as_raw_fd()).unwrap();

    // 5文字送信
    write(c_sock.as_raw_fd(), "HELLO".as_bytes()).unwrap();

    // TCPセッションの終了
    close(c_sock).unwrap();
    close(sock.as_raw_fd()).unwrap();
}