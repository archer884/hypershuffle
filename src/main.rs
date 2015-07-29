extern crate hyper;
extern crate time;

use hyper::Client;
use time::PreciseTime;

const URL: &'static str = "https://www.reddit.com/r/dailyprogrammer/comments/3e0hmh/20150720_challenge_224_easy_shuffling_a_list/";

trait Shuffle {
    type Output: Ord;
    fn next(&mut self) -> Self::Output;
}

struct HyperShuffle {
    url: String,
    client: Client
}

impl HyperShuffle {
    fn new<S: Into<String>>(url: S) -> HyperShuffle {
        HyperShuffle {
            url: url.into(),
            client: Client::new()
        }
    }
}

impl Shuffle for HyperShuffle {
    type Output = i64;

    fn next(&mut self) -> Self::Output {
        let time = PreciseTime::now();
        self.client.get(&self.url).send().ok();
        time.to(PreciseTime::now()).num_microseconds().unwrap_or(3)
    }
}

fn main() {
    let mut shuffle_client = HyperShuffle::new(URL);
    let items = std::env::args().skip(1).collect();
    let shuffled_items = shuffle(&mut shuffle_client, items);

    for item in &shuffled_items {
        println!("{:?}", item);
    }
}

fn shuffle<T, S: Shuffle>(shuffle: &mut S, items: Vec<T>) -> Vec<T> {
    let mut items: Vec<_> = items
        .into_iter()
        .map(|item| (shuffle.next(), item))
        .collect();

    items.sort_by(|a, b| a.0.cmp(&b.0));
    items.into_iter().map(|(_, v)| v).collect()
}
