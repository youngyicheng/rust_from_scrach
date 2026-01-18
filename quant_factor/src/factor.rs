use crate::wide_table::WideTable;
use anyhow::Result;

/// 因子计算器
pub struct FactorCalculator;

impl FactorCalculator {
    /// 计算动量因子
    /// 
    /// # Arguments
    /// * `table` - 宽表数据（包含 close 价格）
    /// * `periods` - 动量周期
    /// 
    /// # Returns
    /// 包含动量因子的宽表
    pub fn calculate_momentum(table: &WideTable, periods: i32) -> Result<WideTable> {
        table.momentum(periods)
    }
    
    /// 计算收益率
    /// 
    /// # Arguments
    /// * `table` - 宽表数据（包含 close 价格）
    /// * `periods` - 收益率周期，默认为 1
    /// 
    /// # Returns
    /// 包含收益率的宽表
    pub fn calculate_returns(table: &WideTable, periods: i32) -> Result<WideTable> {
        table.pct_change(periods)
    }
}
