use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let mut items: Vec<u32> = Vec::new();
    let mut j = 12046;
    print!("e12045 & ");
    (0..25).for_each(|i| {
        let value: u32 = rng.gen_range(0..1000000);
        let value: u32 = rng.gen_range(0..value);
        let value: u32 = rng.gen_range(0..value);
        let value: u32 = rng.gen_range(0..value);
        items.push(value);
        if i % 5 == 4 {
            print!("{} \\\\\ne{} & ", value, j);
            j += 1;
        } else {
            print!("{} & ", value);
        }
    });
    println!("{:?}", items);
}
