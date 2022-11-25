struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_points)
    }
}

fn main() {
    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();

    let s = "Hello, world!";
    let t = ImportantExcerpt { part: &s };
    t.announce_and_return_part("Good job!");
    t.announce_and_return_part1("Good job!");
    dbg!(longest_with_an_announcement("haha", "yoyo", "good job!"));
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // fn level(&self) -> i32 {
    //     3
    // }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
    fn announce_and_return_part1<'b>(&self, announcement: &'b str) -> &'b str
    where
        'a: 'b, // imply that 'a is longer than 'b, because the most powerful is always placed first
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement {}", ann);
    if x.len() >= y.len() { x } else { y }
}
