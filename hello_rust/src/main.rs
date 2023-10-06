fn greet_world() {
    let southern_germany: &str = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let hi_rust: &str = "Hello rust!\n";
    let regions: [&str; 4] = [southern_germany, chinese, english, hi_rust];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}