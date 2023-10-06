fn greet_world() {
    let southern_germany: &str = "Grüß Gott!\n";
    let chinese = "世界，你好.\n";
    let english = "World, hello.\n";
    let hi_rust: &str = "Hello rust!\n";
    let regions: [&str; 4] = [southern_germany, chinese, english, hi_rust];
    for region in regions.iter() {
        print!("{}", &region);
    }
}

fn main() {
    greet_world();
}