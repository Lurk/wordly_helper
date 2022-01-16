use a_thing::dict::Dict;

use std::time::Instant;

fn main() {
    let whole = Timer::new("whole");
    let read = Timer::new("read");
    if let Ok(d) = Dict::from_file("./data/sowpods_5.txt") {
        read.end();
        let f2 = Timer::new("weights");
        let weights = d.get_char_weights();
        f2.end();
        // println!("{:?}", freq2);
        let filter = Timer::new("filters");
        let words = d
            .contains_chars("")
            .not_contains_chars("")
            .positional_contains_chars(&[None, None, None, None, None])
            .positional_not_contains_chars(&[None, None, None, None, None])
            .apply();
        filter.end();
        let mc = Timer::new("most_common");
        println!("{:#?}", words.most_common(&weights, 10));
        mc.end();
    }
    whole.end();
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
        println!("{} is done in: {:?}", self.name, duration);
    }
}
