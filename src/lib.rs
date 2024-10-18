use regex::Regex;
use std::collections::HashMap;
use std::fs::{self};
use std::io::{self};
use std::path::{Path, PathBuf};

use self::model::{Book, Highlight, HighlightLocation, Note};

pub mod model;

pub fn run(clippings: PathBuf, _template: Option<PathBuf>) {
    if let Ok(s) = read_file_string(clippings) {
        let books = parse_clippings(s);

        for (_, book) in books.iter() {
            println!(
                "{} by {} has {} highlights",
                book.title(),
                book.author(),
                book.highlights().len()
            );
        }
    } else {
        panic!("error reading file");
    }
}

fn read_file_string<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_to_string(filename)?
        .replace("\r\n", " ") // clean line endings
        .replace("\u{feff}", "") // clean the BOM
        .split("==========")
        .map(String::from)
        .collect())
}

fn parse_clippings(clippings: Vec<String>) -> HashMap<String, Book> {
    let mut library: HashMap<String, Book> = HashMap::new();

    let re_highlights = Regex::new(r"\s*(?<title>.*)\s\((?<author>.*,.*)\) - Your Highlight on page (?<page>\d+) \| location (?<loc_start>\d+)-(?<loc_end>\d+) \| Added on (?<timestamp>.+ \d{4} \d{2}:\d{2}:\d{2})\s+(?<quote>.*)\s*").unwrap();

    let re_note = Regex::new(r"\s*(?<title>.*)\s\((?<author>.*,.*)\) - Your Note on page (?<page>\d+) \| location (?<location>\d+) \| Added on (?<timestamp>.+ \d{4} \d{2}:\d{2}:\d{2})\s+(?<note>.*)\s*").unwrap();

    for entry in clippings.iter() {
        // if we have a highlight
        // check for highlights first as they'll likely be more common
        if let Some((_, [title, author, page, loc_start, loc_end, _timestamp, quote])) =
            re_highlights.captures(entry).map(|c| c.extract())
        {
            // ensure that we have the book in our library
            if !library.contains_key(title) {
                library.insert(
                    title.to_string(),
                    Book::new(title.to_string(), author.to_string()),
                );
            }

            library
                .get_mut(title)
                .unwrap()
                .add_highlight(Highlight::new(
                    page.parse().unwrap(),
                    HighlightLocation::new(loc_start.parse().unwrap(), loc_end.parse().unwrap()),
                    quote.to_string(),
                ));

        // If we don't have a highlight, check for a note
        } else if let Some((_, [title, _author, page, location, _timestamp, note])) =
            re_note.captures(entry).map(|c| c.extract())
        {
            library.get_mut(title).unwrap().add_note(Note::new(
                page.parse().unwrap(),
                location.parse().unwrap(),
                note.trim().to_string(),
            ));
        }
    }

    println!("library length: {}", library.len());
    library
}

#[cfg(test)]
mod tests {
    use crate::model::HighlightLocation;
    use crate::parse_clippings;

    fn get_input() -> Vec<String> {
        let input: Vec<String> = "\
The 5 AM Club: Own Your Morning. Elevate Your Life. (Sharma, Robin)
- Your Highlight on page 90 | location 1370-1371 | Added on Sunday, 20 August 2023 21:13:59

For most people the truth is that it’s all about the path of least resistance. Getting what they need to get done fast and just sneaking by. Mailing it in instead of bringing it on.
==========
﻿The 5 AM Club: Own Your Morning. Elevate Your Life. (Sharma, Robin)
- Your Highlight on page 93 | location 1432-1433 | Added on Monday, 21 August 2023 21:36:48

The Top 5% go granular versus applying a superficial mindset to their daily attitudes, behaviors and activities.”
==========
﻿The 5 AM Club: Own Your Morning. Elevate Your Life. (Sharma, Robin)
- Your Highlight on page 95 | location 1460-1462 | Added on Monday, 21 August 2023 22:01:40

Gamble everything for love, if you are a true human being. If not, leave this gathering. Half-heartedness doesn’t reach into majesty.
==========
﻿The 5 AM Club: Own Your Morning. Elevate Your Life. (Sharma, Robin)
- Your Highlight on page 100 | location 1536-1537 | Added on Monday, 21 August 2023 22:14:52

while growth as a producer and as a person can be hard—it truly is the finest work a human being can ever do.
==========
The 5 AM Club: Own Your Morning. Elevate Your Life. (Sharma, Robin)
- Your Highlight on page 88 | location 1348-1353 | Added on Monday, 21 August 2023 22:38:39

it. Picture how mysterious all this seemed. “A message in a bottle,” declared the billionaire happily. He started clapping his hands like a little tyke. He sure was an abnormal and totally wonderful character. “This conveniently sets the tone for my mentoring session with you this morning,” he added. The industrialist then lifted the vessel, unscrewed the cap and pulled out the fabric, which had the framework below stitched onto it:
==========
The 5 AM Club: Own Your Morning. Elevate Your Life. (Sharma, Robin)
- Your Note on page 100 | location 1537 | Added on Friday, 18 October 2024 11:49:48

Smidgen the pigeon 
==========
            ".to_string()
            .replace("\r\n", " ")
            .replace("\n", " ")
            .replace("\u{feff}", "") // clean the BOM
            .split("==========")
            .map(String::from)
            .collect();

        dbg!(&input);
        input
    }

    #[test]
    fn model() {
        let library = parse_clippings(get_input());
        assert_eq!(1, library.len());

        let book = library.get("The 5 AM Club: Own Your Morning. Elevate Your Life.").unwrap();

        assert_eq!("Sharma, Robin".to_string(), book.author());
        assert_eq!(5, book.highlights().len());

        let loc = HighlightLocation::new(1536, 1537);
        let hl = book.highlights().get(&loc).unwrap();

        assert_eq!(&100, hl.page());

        let note = hl.note().clone().unwrap();
        assert_eq!("Smidgen the pigeon", note.content());
    }
}
