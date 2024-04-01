const BIBLE_BOOKS: &[&str] = &[
    "1 Peter",
    "2 Peter",
    "1 John",
    "2 John",
    "3 John",
    "1 Samuel",
    "2 Samuel",
    "1 Kings",
    "2 Kings",
    "1 Chronicles",
    "2 Chronicles",
    "1 Corinthians",
    "2 Corinthians",
    "1 Thessalonians",
    "2 Thessalonians",
    "1 Timothy",
    "2 Timothy",
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Deuteronomy",
    "Joshua",
    "Judges",
    "Ruth",
    "Ezra",
    "Nehemiah",
    "Esther",
    "Job",
    "Psalms",
    "Proverbs",
    "Ecclesiastes",
    "Song of Solomon",
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Ezekiel",
    "Daniel",
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
    "Matthew",
    "Mark",
    "Luke",
    "John",
    "Acts",
    "Romans",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "Titus",
    "Philemon",
    "Hebrews",
    "James",
    "Jude",
    "Revelation",
];

const ALTERNATIVE_BOOK_NAMES: &[(&str, &str)] = &[
    ("1Pet", "1 Peter"),
    ("2Pet", "2 Peter"),
    ("1Jn", "1 John"),
    ("2Jn", "2 John"),
    ("3Jn", "3 John"),
    ("1Pet", "1 Peter"),
    ("2Pet", "2 Peter"),
    ("1Jn", "1 John"),
    ("2Jn", "2 John"),
    ("3Jn", "3 John"),
    ("1Sam", "1 Samuel"),
    ("2Sam", "2 Samuel"),
    ("1Ki", "1 Kings"),
    ("2Ki", "2 Kings"),
    ("1Chr", "1 Chronicles"),
    ("2Chr", "2 Chronicles"),
    ("1Sam", "1 Samuel"),
    ("2Sam", "2 Samuel"),
    ("1Ki", "1 Kings"),
    ("2Ki", "2 Kings"),
    ("1Chr", "1 Chronicles"),
    ("2Chr", "2 Chronicles"),
    ("1Cor", "1 Corinthians"),
    ("2Cor", "2 Corinthians"),
    ("1Cor", "1 Corinthians"),
    ("2Cor", "2 Corinthians"),
    ("1Thess", "1 Thessalonians"),
    ("2Thess", "2 Thessalonians"),
    ("1Tim", "1 Timothy"),
    ("2Tim", "2 Timothy"),
    ("1Thess", "1 Thessalonians"),
    ("2Thess", "2 Thessalonians"),
    ("1Tim", "1 Timothy"),
    ("2Tim", "2 Timothy"),
    ("1 Pet", "1 Peter"),
    ("2 Pet", "2 Peter"),
    ("1 Jn", "1 John"),
    ("2 Jn", "2 John"),
    ("3 Jn", "3 John"),
    ("1 Pet", "1 Peter"),
    ("2 Pet", "2 Peter"),
    ("1 Jn", "1 John"),
    ("2 Jn", "2 John"),
    ("3 Jn", "3 John"),
    ("1 Sam", "1 Samuel"),
    ("2 Sam", "2 Samuel"),
    ("1 Ki", "1 Kings"),
    ("2 Ki", "2 Kings"),
    ("1 Chr", "1 Chronicles"),
    ("2 Chr", "2 Chronicles"),
    ("1 Sam", "1 Samuel"),
    ("2 Sam", "2 Samuel"),
    ("1 Ki", "1 Kings"),
    ("2 Ki", "2 Kings"),
    ("1 Chr", "1 Chronicles"),
    ("2 Chr", "2 Chronicles"),
    ("1 Cor", "1 Corinthians"),
    ("2 Cor", "2 Corinthians"),
    ("1 Cor", "1 Corinthians"),
    ("2 Cor", "2 Corinthians"),
    ("1 Thess", "1 Thessalonians"),
    ("2 Thess", "2 Thessalonians"),
    ("1 Tim", "1 Timothy"),
    ("2 Tim", "2 Timothy"),
    ("1 Thess", "1 Thessalonians"),
    ("2 Thess", "2 Thessalonians"),
    ("1 Tim", "1 Timothy"),
    ("2 Tim", "2 Timothy"),
    ("Psalm", "Psalms"),
    ("Pslam", "Psalms"),
    ("Pslams", "Psalms"),
    ("Gen", "Genesis"),
    ("Ex", "Exodus"),
    ("Lev", "Leviticus"),
    ("Num", "Numbers"),
    ("Deut", "Deuteronomy"),
    ("Josh", "Joshua"),
    ("Judg", "Judges"),
    ("Ruth", "Ruth"),
    ("Ezr", "Ezra"),
    ("Neh", "Nehemiah"),
    ("Est", "Esther"),
    ("Prov", "Proverbs"),
    ("Eccl", "Ecclesiastes"),
    ("Song", "SongofSolomon"),
    ("Isa", "Isaiah"),
    ("Jer", "Jeremiah"),
    ("Lam", "Lamentations"),
    ("Ezek", "Ezekiel"),
    ("Dan", "Daniel"),
    ("Hos", "Hosea"),
    ("Am", "Amos"),
    ("Ob", "Obadiah"),
    ("Jon", "Jonah"),
    ("Mic", "Micah"),
    ("Nah", "Nahum"),
    ("Hab", "Habakkuk"),
    ("Zeph", "Zephaniah"),
    ("Hag", "Haggai"),
    ("Zech", "Zechariah"),
    ("Mal", "Malachi"),
    ("Matt", "Matthew"),
    ("Mk", "Mark"),
    ("Lk", "Luke"),
    ("Jn", "John"),
    ("Rom", "Romans"),
    ("Gal", "Galatians"),
    ("Eph", "Ephesians"),
    ("Phil", "Philippians"),
    ("Col", "Colossians"),
    ("Tit", "Titus"),
    ("Phlm", "Philemon"),
    ("Heb", "Hebrews"),
    ("Jas", "James"),
    ("Rev", "Revelation"),
];

const OSIS_BOOK_NAMES: &[(&str, &str)] = &[
    ("Genesis", "Gen"),
    ("Exodus", "Exod"),
    ("Leviticus", "Lev"),
    ("Numbers", "Num"),
    ("Deuteronomy", "Deut"),
    ("Joshua", "Josh"),
    ("Judges", "Judg"),
    ("Ruth", "Ruth"),
    ("1 Samuel", "1Sam"),
    ("2 Samuel", "2Sam"),
    ("1 Kings", "1Kgs"),
    ("2 Kings", "2Kgs"),
    ("1 Chronicles", "1Chr"),
    ("2 Chronicles", "2Chr"),
    ("Ezra", "Ezra"),
    ("Nehemiah", "Neh"),
    ("Esther", "Esth"),
    ("Job", "Job"),
    ("Psalms", "Ps"),
    ("Proverbs", "Prov"),
    ("Ecclesiastes", "Eccl"),
    ("Song of Solomon", "Song"),
    ("Isaiah", "Isa"),
    ("Jeremiah", "Jer"),
    ("Lamentations", "Lam"),
    ("Ezekiel", "Ezek"),
    ("Daniel", "Dan"),
    ("Hosea", "Hos"),
    ("Joel", "Joel"),
    ("Amos", "Amos"),
    ("Obadiah", "Obad"),
    ("Jonah", "Jonah"),
    ("Micah", "Mic"),
    ("Nahum", "Nah"),
    ("Habakkuk", "Hab"),
    ("Zephaniah", "Zeph"),
    ("Haggai", "Hag"),
    ("Zechariah", "Zech"),
    ("Malachi", "Mal"),
    ("Matthew", "Matt"),
    ("Mark", "Mark"),
    ("Luke", "Luke"),
    ("John", "John"),
    ("Acts", "Acts"),
    ("Romans", "Rom"),
    ("1 Corinthians", "1Cor"),
    ("2 Corinthians", "2Cor"),
    ("Galatians", "Gal"),
    ("Ephesians", "Eph"),
    ("Philippians", "Phil"),
    ("Colossians", "Col"),
    ("1 Thessalonians", "1Thess"),
    ("2 Thessalonians", "2Thess"),
    ("1 Timothy", "1Tim"),
    ("2 Timothy", "2Tim"),
    ("Titus", "Titus"),
    ("Philemon", "Phlm"),
    ("Hebrews", "Heb"),
    ("James", "Jas"),
    ("1 Peter", "1Pet"),
    ("2 Peter", "2Pet"),
    ("1 John", "1John"),
    ("2 John", "2John"),
    ("3 John", "3John"),
    ("Jude", "Jude"),
    ("Revelation", "Rev"),
];

pub fn parse_bible_reference(query: &str) -> Option<(String, u32, Option<u32>, Option<u32>)> {
    // Convert the query to lowercase for case-insensitive matching
    let query_lower = query.to_lowercase();

    // Check for main book names
    for book in BIBLE_BOOKS {
        if let Some(index) = query_lower.find(book.to_lowercase().as_str()) {
            let remaining_query = &query_lower[index + book.len()..].trim();
            if let Some((chapter, verse_start, verse_end)) =
                parse_chapter_and_verses(remaining_query)
            {
                if let Some(osis_name) = OSIS_BOOK_NAMES
                    .iter()
                    .find(|(readable, _)| readable == book)
                    .map(|(_, osis)| *osis)
                {
                    return Some((osis_name.to_string(), chapter, verse_start, verse_end));
                }
            }
        }
    }

    // Check for alternative book names
    for (alt_name, book) in ALTERNATIVE_BOOK_NAMES {
        if let Some(index) = query_lower.find(alt_name.to_lowercase().as_str()) {
            let remaining_query = &query_lower[index + alt_name.len()..].trim();
            if let Some((chapter, verse_start, verse_end)) =
                parse_chapter_and_verses(remaining_query)
            {
                if let Some(osis_name) = OSIS_BOOK_NAMES
                    .iter()
                    .find(|(readable, _)| readable == book)
                    .map(|(_, osis)| *osis)
                {
                    return Some((osis_name.to_string(), chapter, verse_start, verse_end));
                }
            }
        }
    }

    // Check for OSIS book names
    for (_, osis_name) in OSIS_BOOK_NAMES {
        if let Some(index) = query_lower.find(osis_name.to_lowercase().as_str()) {
            let remaining_query = &query_lower[index + osis_name.len()..].trim();
            if let Some((chapter, verse_start, verse_end)) =
                parse_chapter_and_verses(remaining_query)
            {
                return Some((osis_name.to_string(), chapter, verse_start, verse_end));
            }
        }
    }

    None
}

pub fn format_bible_reference(
    reference: Option<(String, u32, Option<u32>, Option<u32>)>,
) -> Option<String> {
    reference.map(|(book, chapter, verse_start, verse_end)| {
        let mut formatted = format!("{}.{}", book, chapter);

        if let Some(start) = verse_start {
            formatted.push_str(&format!(".{}", start));

            if let Some(end) = verse_end {
                formatted.push_str(&format!("-{}", end));
            }
        }

        formatted
    })
}


fn parse_chapter_and_verses(query: &str) -> Option<(u32, Option<u32>, Option<u32>)> {
    let numbers: Vec<u32> = query
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse().ok())
        .collect();

    match numbers.len() {
        1 => Some((numbers[0], None, None)),
        2 => Some((numbers[0], Some(numbers[1]), None)),
        3 => Some((numbers[0], Some(numbers[1]), Some(numbers[2]))),
        _ => None,
    }
}
