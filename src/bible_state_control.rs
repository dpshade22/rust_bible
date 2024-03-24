pub fn search_verses_by_keyword<'a>(verses: &'a [Verse]) -> Vec<Verse> {
    let search_word = format!(" {} ", word.to_lowercase());
    let book_refs: [&mut str; 66] = [""; 66];

    debug!("Searching {word}");
    book_refs
        .iter()
        .filter(|verse| verse.book != book_refs[x])
        .cloned()
        .collect()
}
