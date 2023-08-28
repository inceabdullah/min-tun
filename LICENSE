use std::fs;

pub fn find_min_available_tun() -> Option<String> {
    let path = "/sys/class/net/";

    // Read the directory
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    // Collect existing tun device names
    let mut existing_tun_devices: Vec<u32> = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("tun") {
                if let Ok(index) = name_str[3..].parse::<u32>() {
                    existing_tun_devices.push(index);
                }
            }
        }
    }

    // Sort the indices
    existing_tun_devices.sort();

    // Find the first unused index
    let mut min_available_tun = 0;
    for &index in &existing_tun_devices {
        if index == min_available_tun {
            min_available_tun += 1;
        } else {
            break;
        }
    }

    Some(format!("tun{}", min_available_tun))
}