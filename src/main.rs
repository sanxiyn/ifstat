fn main() {
    let networks = sysinfo::Networks::new_with_refreshed_list();
    let map = networks.list();
    let mut names = map.keys().collect::<Vec<_>>();
    names.sort();
    for name in &names {
        let name: &str = &name;
        let network = &map[name];
        let rx_packets = network.total_packets_received();
        let rx_bytes = network.total_received();
        let rx_human = humansize::format_size(rx_bytes, humansize::BINARY);
        let tx_packets = network.total_packets_transmitted();
        let tx_bytes = network.total_transmitted();
        let tx_human = humansize::format_size(tx_bytes, humansize::BINARY);
        println!("{}", name);
        println!("  RX packets {} bytes {} ({})",
            rx_packets, rx_bytes, rx_human);
        println!("  TX packets {} bytes {} ({})",
            tx_packets, tx_bytes, tx_human);
    }
}
