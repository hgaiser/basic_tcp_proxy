use basic_tcp_proxy::TcpProxy;

fn main() {
    log::info!("starting client");
    let _proxy = TcpProxy::new(9091, "10.200.1.2:9091".parse().unwrap(), false);
    loop {}
}
