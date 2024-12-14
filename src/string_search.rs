pub fn hash_search(word: &[u8], message: &[u8]) -> Option<usize> {
    let word_hash = word
        .iter()
        .fold(0, |acc: u32, val| acc.wrapping_add(*val as u32));
    let mut chunk_hash = message[..(word.len() - 1)]
        .iter()
        .fold(0, |acc: u32, val| acc.wrapping_add(*val as u32));
    for i in 0..(message.len() - word.len() + 1) {
        chunk_hash = chunk_hash.wrapping_add(message[i + word.len() - 1] as u32);
        if word_hash == chunk_hash {
            if word
                .iter()
                .zip(message[i..(i + word.len())].iter())
                .all(|(w, m)| w == m)
            {
                return Some(i);
            }
        }
        chunk_hash = chunk_hash.saturating_sub(message[i] as u32);
    }
    None
}

#[test]
fn test_string_search() {
    let message = b"thequickredfoxjumped";
    assert_eq!(hash_search(b"the", message), Some(0));
    assert_eq!(hash_search(b"fox", message), Some(11));
    assert_eq!(hash_search(b"jumped", message), Some(14));
    assert_eq!(hash_search(b"rabbit", message), None);
}
