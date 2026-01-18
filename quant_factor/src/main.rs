use quant_factor::{WideTable, FactorCalculator};
use polars::prelude::*;
use chrono::NaiveDate;
use anyhow::Result;

fn main() -> Result<()> {
    println!("量化因子计算系统");
    println!("==================");
    
    // 创建示例数据
    let dates = vec![
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 4).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
    ];
    
    // 模拟股票收盘价数据
    let stock_000001 = vec![10.0, 10.2, 10.5, 10.3, 10.8];
    let stock_000002 = vec![20.0, 20.5, 21.0, 20.8, 21.5];
    let stock_600000 = vec![15.0, 15.3, 15.1, 15.5, 15.9];
    
    // 创建 DataFrame
    let df = DataFrame::new(vec![
        Series::new("date", dates),
        Series::new("000001", stock_000001),
        Series::new("000002", stock_000002),
        Series::new("600000", stock_600000),
    ])?;
    
    println!("\n原始价格数据:");
    println!("{}", df);
    
    // 创建宽表
    let table = WideTable::new(df, "date")?;
    
    // 计算收益率
    println!("\n计算收益率 (pct_change):");
    let returns = FactorCalculator::calculate_returns(&table, 1)?;
    println!("{}", returns.df());
    
    // 计算动量因子（过去 3 期）
    println!("\n计算动量因子 (momentum, periods=3):");
    let momentum = FactorCalculator::calculate_momentum(&table, 3)?;
    println!("{}", momentum.df());
    
    // 保存结果到 CSV（可选）
    // returns.to_csv("returns.csv")?;
    // momentum.to_csv("momentum.csv")?;
    
    Ok(())
}
