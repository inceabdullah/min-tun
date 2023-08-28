fn main() {
    match min_tun::find_min_available_tun() {  // Replace 'min_tun' with your crate name if different
        Some(tun_name) => println!("Minimum available tun device: {}", tun_name),
        None => println!("Could not find an available tun device"),
    }
}
