pub fn extract_target_triple(name: &str) -> Option<String> {
    let name = name
        .trim_end_matches(".tar.gz")
        .trim_end_matches(".tar.xz")
        .trim_end_matches(".tar.bz2")
        .trim_end_matches(".zip");

    let parts: Vec<&str> = name.split('-').collect();
    let arch_candidates = ["x86_64", "i686", "i386", "aarch64", "arm64"];

    for (i, part) in parts.iter().enumerate() {
        if arch_candidates.contains(part) {
            return Some(parts[i..].join("-"));
        }
    }
    None
}