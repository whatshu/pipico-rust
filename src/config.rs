//! 系统配置常量

/// UART 配置
pub mod uart {
    /// 波特率
    pub const BAUD_RATE: u32 = 115200;
    
    /// TX 引脚（GPIO0）
    pub const TX_PIN: u8 = 0;
    
    /// RX 引脚（GPIO1）
    pub const RX_PIN: u8 = 1;
}

/// 任务配置
pub mod task {
    /// Core 0 心跳间隔（毫秒）
    pub const CORE0_INTERVAL_MS: u64 = 1000;
    
    /// Core 1 心跳间隔（毫秒）
    pub const CORE1_INTERVAL_MS: u64 = 1500;
    
    /// Core 0 里程碑间隔
    pub const CORE0_MILESTONE: u32 = 10;
    
    /// Core 1 检查点间隔
    pub const CORE1_CHECKPOINT: u32 = 5;
}

/// Core 1 栈大小
pub const CORE1_STACK_SIZE: usize = 4096;

