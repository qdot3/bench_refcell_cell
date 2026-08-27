use cache_size::{l1_cache_line_size, l1_cache_size};

/// FIXME: This supports x86 architectures only for now
pub fn cache_info() -> String {
    let l1_size = match l1_cache_size() {
        Some(n) => {
            if n.trailing_zeros() >= 20 {
                format!("{} MiB", n >> 20)
            } else if n.trailing_zeros() >= 10 {
                format!("{} KiB", n >> 10)
            } else {
                format!("{} B", n)
            }
        }
        None => "NA".to_string(),
    };
    let l1_line_size = match l1_cache_line_size() {
        Some(n) => format!("{n} B"),
        None => "NA".to_string(),
    };
    let word_size = format!("{} B", std::mem::size_of::<usize>());

    format!("L1: {l1_size}, L1 Line: {l1_line_size}, word: {word_size}")
}
