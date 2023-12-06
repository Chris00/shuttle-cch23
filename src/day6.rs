
pub async fn app(txt: String) -> String {
    let elf = txt.matches("elf").count();
    let elf_shelf = txt.matches("elf on a shelf").count();
    let shelf = txt.match_indices("shelf")
        .filter(|&(i, _)| {
            i < 9 || &txt[i-9 .. i] != "elf on a "
        })
        .count();
    format!("{{\"elf\":{elf}, \"elf on a shelf\": {elf_shelf}, \
             \"shelf with no elf on it\": {shelf}}}")
}
