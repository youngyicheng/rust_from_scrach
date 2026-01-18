use polars::prelude::*;
use chrono::NaiveDate;
use anyhow::Result;

/// 宽表数据结构
/// - index: 时间（DateTime）
/// - columns: 股票代码
/// - values: 价格数据（如 close）
pub struct WideTable {
    /// DataFrame，行索引为时间，列为股票代码
    df: DataFrame,
    /// 时间列名
    time_col: String,
}

impl WideTable {
    /// 创建新的宽表
    /// 
    /// # Arguments
    /// * `df` - DataFrame，必须包含时间列和多个股票列
    /// * `time_col` - 时间列的名称
    pub fn new(df: DataFrame, time_col: impl Into<String>) -> Result<Self> {
        let time_col = time_col.into();
        
        // 验证时间列存在
        if !df.column(&time_col).is_ok() {
            return Err(anyhow::anyhow!("时间列 '{}' 不存在", time_col));
        }
        
        Ok(Self { df, time_col })
    }
    
    /// 从 CSV 文件加载宽表数据
    /// 
    /// # Arguments
    /// * `path` - CSV 文件路径
    /// * `time_col` - 时间列名称
    pub fn from_csv(path: impl AsRef<std::path::Path>, time_col: impl Into<String>) -> Result<Self> {
        let time_col = time_col.into();
        let df = LazyFrame::scan_csv(path, ScanArgsCSV::default())
            .collect()?;
        
        Self::new(df, time_col)
    }
    
    /// 获取 DataFrame
    pub fn df(&self) -> &DataFrame {
        &self.df
    }
    
    /// 获取时间列名
    pub fn time_col(&self) -> &str {
        &self.time_col
    }
    
    /// 计算百分比变化（收益率）
    /// 
    /// # Arguments
    /// * `periods` - 计算周期，例如 1 表示计算 1 期的收益率
    /// 
    /// # Returns
    /// 返回新的 WideTable，包含收益率数据
    pub fn pct_change(&self, periods: i32) -> Result<WideTable> {
        let mut df = self.df.clone();
        
        // 按时间列排序
        df = df.sort([&self.time_col], SortOptions::default())?;
        
        // 获取所有列名（排除时间列）
        let stock_cols: Vec<String> = df
            .get_column_names()
            .iter()
            .filter(|&col| col != &self.time_col)
            .map(|s| s.to_string())
            .collect();
        
        // 对每个股票列计算 pct_change
        let mut lazy_df = df.lazy();
        
        for col_name in &stock_cols {
            let pct_col = format!("{}_pct_change_{}", col_name, periods);
            // 使用 shift 和除法来计算百分比变化: (current - previous) / previous * 100
            lazy_df = lazy_df.with_columns([
                ((col(col_name) - col(col_name).shift(lit(periods))) 
                    / col(col_name).shift(lit(periods)) 
                    * lit(100.0))
                    .alias(&pct_col)
            ]);
        }
        
        let new_df = lazy_df.collect()?;
        
        Ok(WideTable {
            df: new_df,
            time_col: self.time_col.clone(),
        })
    }
    
    /// 计算动量因子
    /// 
    /// 动量因子通常定义为过去 N 期的累计收益率
    /// 
    /// # Arguments
    /// * `periods` - 动量周期，例如 20 表示过去 20 期的动量
    /// 
    /// # Returns
    /// 返回包含动量因子的新 WideTable
    pub fn momentum(&self, periods: i32) -> Result<WideTable> {
        // 先计算收益率
        let ret_table = self.pct_change(1)?;
        
        let mut df = ret_table.df().clone();
        
        // 按时间列排序
        df = df.sort([&self.time_col], SortOptions::default())?;
        
        // 获取所有收益率列名
        let ret_cols: Vec<String> = df
            .get_column_names()
            .iter()
            .filter(|&col| col != &self.time_col && col.contains("_pct_change_1"))
            .map(|s| s.to_string())
            .collect();
        
        // 对每个收益率列计算滚动求和（动量）
        let mut lazy_df = df.lazy();
        
        for ret_col in &ret_cols {
            // 提取原始列名
            let base_col = ret_col.replace("_pct_change_1", "");
            let momentum_col = format!("{}_momentum_{}", base_col, periods);
            
            // 计算过去 periods 期的累计收益率
            // 使用 rolling_sum 计算滚动窗口内的累计收益率
            lazy_df = lazy_df.with_columns([
                col(ret_col)
                    .rolling_sum(RollingOptionsFixedWindow {
                        window_size: periods as usize,
                        min_periods: 1,
                        center: false,
                    })
                    .alias(&momentum_col)
            ]);
        }
        
        let new_df = lazy_df.collect()?;
        
        Ok(WideTable {
            df: new_df,
            time_col: self.time_col.clone(),
        })
    }
    
    /// 保存到 CSV 文件
    pub fn to_csv(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        CsvWriter::new(&mut file)
            .include_header(true)
            .finish(&mut self.df.clone())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wide_table_creation() {
        // 创建测试数据
        let dates = vec![
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
        ];
        
        let stock_a = vec![100.0, 102.0, 104.0];
        let stock_b = vec![50.0, 51.0, 49.0];
        
        let df = DataFrame::new(vec![
            Series::new("date", dates),
            Series::new("stock_A", stock_a),
            Series::new("stock_B", stock_b),
        ]).unwrap();
        
        let table = WideTable::new(df, "date").unwrap();
        assert_eq!(table.time_col(), "date");
    }
}
