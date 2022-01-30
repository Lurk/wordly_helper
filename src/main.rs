use a_thing::file::from_file;
use a_thing::filters::contains_chars;

use std::io::Error;
use std::time::Instant;

fn main() -> Result<(), Error> {
    let whole = Timer::new("whole");
    let read = Timer::new("read");
    if let Ok(d) = from_file("./data/sowpods_5.txt") {
        read.end();
        let filter = Timer::new("filters");
        let words = contains_chars(&d, "")
            .not_contains_chars("")
            .positional_contains_chars(&[None, None, None, None, None])
            .positional_not_contains_chars(&[None, None, None, None, None])
            .apply();

        filter.end();

        println!("{:#?}", words.iter().take(10).collect::<Vec<&String>>());
    }
    whole.end();
    Ok(())
}

struct Timer<'a> {
    start: std::time::Instant,
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            start: Instant::now(),
            name,
        }
    }

    pub fn end(self) {
        let duration = self.start.elapsed();
        println!("{}: {:?}", self.name, duration);
    }
}
