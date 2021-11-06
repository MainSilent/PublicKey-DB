pub fn sort() -> &'static [u8] {
    "value".as_bytes()
}

// fn sort_keys(value: &str, path: &str) {

//     // Sort the data
//     if fs::metadata(path).is_ok() {
//         let file = fs::read(path).unwrap();
//         let mut keys: Vec<&[u8]> = file.chunks(32).collect();
//         keys.push(&new_key);
//         keys.sort();
        
//         let new_keys = keys.iter()
//             .flat_map(|arr| arr.iter())
//             .cloned()
//             .collect::<Vec<u8>>();

//         add_to_file(&new_keys, path);
//     } else {
//         add_to_file(&new_key, path);
//     }
// }