use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个DataFrame
    let s0 = Series::new("days", [0, 1, 2].as_ref());
    let s1 = Series::new("temp", [22.1, 19.9, 7.].as_ref());

    let df: DataFrame = DataFrame::new(vec![s0, s1])?;
    // 打印DataFrame的内容
    println!("{}", df);

    // 选择单个列
    let days = df.column("days").unwrap();
    println!("{}", days);

    // 计算一个新的列
    let new_days: Series = days.cast(&DataType::Float64)? * 2.5;
    println!("{}", new_days);
    let df = df.select(["days"])?;
    println!("{}", df);
    Ok(())
}
