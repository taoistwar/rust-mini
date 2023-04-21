fn main() {
    let data = [
        [859, 10760, 366, 84602, 57376],
        [15132, 1164, 1053, 1524, 10774],
        [1968, 69, 35412, 350115, 44576],
        [41174, 11892, 57008, 109663, 2820],
        [165962, 14359, 258886, 6, 7823],
    ];
    let mut dst = [[0.0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            let total: i32 = data[i].iter().sum();
            let pro = (data[i][j] as f64) / (total as f64);
            dst[i][j] = pro;
        }
    }
    let mut event = 12045;
    for i in 0..5 {
        print!("e{} & ", event);
        for j in 0..5 {
            print!("{:.3} & ", dst[i][j]);
        }
        println!(" \\\\");
        event += 1;
    }

    println!("{:?}", dst);
}
