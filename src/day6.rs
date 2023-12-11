
pub async fn app(txt: String) -> String {
    let elf = txt.matches("elf").count();
    // Take care of possible overlapping patterns (which count here)
    let elf_shelf = txt.match_indices("elf ")
        .filter(|&(i, _)| {
            i + 14 <= txt.len()
                && &txt[i + 4 .. i + 14] == "on a shelf"
        })
        .count();
    let shelf = txt.match_indices("shelf")
        .filter(|&(i, _)| {
            i < 9 || &txt[i-9 .. i] != "elf on a "
        })
        .count();
    format!("{{\"elf\":{elf}, \"elf on a shelf\": {elf_shelf}, \
             \"shelf with no elf on it\": {shelf}}}")
}
