use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Note {
    page: u64,
    location: u64,
    content: String,
}

impl Note {
    pub fn new(page: u64, location: u64, content: String) -> Self {
        Note {
            page,
            location,
            content,
        }
    }

    pub fn page(&self) -> &u64 {
        &self.page
    }

    pub fn location(&self) -> &u64 {
        &self.location
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HighlightLocation(u64, u64);

impl HighlightLocation {
    pub fn new(start: u64, end: u64) -> Self {
        HighlightLocation(start, end)
    }

    pub fn contains_location(&self, location: u64) -> bool {
        location == self.0
            || location == self.1
            || (location > &self.0 - 1 && location < &self.1 + 1)
    }
}

#[derive(Debug)]
pub struct Highlight {
    page: u64,
    location: HighlightLocation,
    quote: Option<String>,
    note: Option<Note>,
}

impl Highlight {
    pub fn new(page: u64, location: HighlightLocation, quote: String) -> Self {
        Highlight {
            page,
            location,
            quote: Some(quote),
            note: None,
        }
    }

    pub fn location(&self) -> &HighlightLocation {
        &self.location
    }

    pub fn page(&self) -> &u64 {
        &self.page
    }

    pub fn note(&self) -> &Option<Note> {
        &self.note
    }

    pub fn add_quote(&mut self, quote: String) {
        self.quote = Some(quote);
    }

    fn add_note(&mut self, note: Note) {
        self.note = Some(note);
    }
}

#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    highlights: BTreeMap<HighlightLocation, Highlight>,
}

impl Book {
    pub fn new(title: String, author: String) -> Self {
        Book {
            title,
            author,
            highlights: BTreeMap::new(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn highlights(&self) -> &BTreeMap<HighlightLocation, Highlight> {
        &self.highlights
    }

    pub fn add_highlight(&mut self, highlight: Highlight) {
        self.highlights
            .insert(highlight.location().to_owned(), highlight);
    }

    pub fn add_note(&mut self, note: Note) {
        let page = note.page().to_owned();
        let location = note.location().to_owned();

        // assuming that a highlight can only contain a single note
        self.highlights
            .iter_mut()
            .filter(|(loc, hl)| *hl.page() == page && loc.contains_location(location))
            .map(|(_k, v)| v)
            .for_each(|hl| hl.add_note(note.clone()));
    }
}
