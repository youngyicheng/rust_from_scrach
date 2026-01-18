# 使用说明

## 安装 Rust

如果还没有安装 Rust，请先安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## 运行项目

```bash
cd quant_factor
cargo run
```

## 项目功能

### 1. 宽表数据结构 (WideTable)

支持时间索引、股票列的数据格式：

```rust
use quant_factor::WideTable;
use polars::prelude::*;
use chrono::NaiveDate;

// 创建数据
let dates = vec![
    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
];

let stock_a = vec![100.0, 102.0];
let stock_b = vec![50.0, 51.0];

let df = DataFrame::new(vec![
    Series::new("date", dates),
    Series::new("000001", stock_a),
    Series::new("000002", stock_b),
])?;

let table = WideTable::new(df, "date")?;
```

### 2. 计算收益率 (pct_change)

```rust
// 计算 1 期收益率
let returns = table.pct_change(1)?;
```

### 3. 计算动量因子

```rust
// 计算过去 20 期的动量因子
let momentum = table.momentum(20)?;
```

### 4. 从 CSV 加载数据

```rust
let table = WideTable::from_csv("data.csv", "date")?;
```

### 5. 保存结果

```rust
returns.to_csv("returns.csv")?;
momentum.to_csv("momentum.csv")?;
```

## 数据格式示例

CSV 文件格式：

```csv
date,000001,000002,600000
2024-01-01,10.0,20.0,15.0
2024-01-02,10.2,20.5,15.3
2024-01-03,10.5,21.0,15.1
```

## 注意事项

- 确保时间列按时间顺序排列
- 股票列应该是数值类型（浮点数）
- 动量因子的计算基于收益率，所以会先计算收益率再计算动量
