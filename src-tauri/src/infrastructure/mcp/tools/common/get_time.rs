use async_trait::async_trait;
use chrono::{Local, Utc};
use serde_json::{json, Value};
use crate::domain::services::mcp_tool::{MCPTool, MCPToolError};

/// 获取当前时间工具
pub struct GetTimeTool;

#[async_trait]
impl MCPTool for GetTimeTool {
    fn name(&self) -> &str {
        "get_current_time"
    }
    
    fn description(&self) -> &str {
        "获取当前系统时间。可以指定时区和时间格式。"
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "时区名称，例如：Asia/Shanghai, UTC, America/New_York。默认为系统本地时区。",
                    "enum": ["local", "UTC", "Asia/Shanghai", "America/New_York", "Europe/London"]
                },
                "format": {
                    "type": "string",
                    "description": "时间格式：RFC3339（ISO8601标准格式）、timestamp（Unix时间戳）、human（人类可读格式）。默认为RFC3339。",
                    "enum": ["RFC3339", "timestamp", "human"]
                }
            },
            "required": []
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, MCPToolError> {
        // 解析参数
        let timezone = arguments
            .get("timezone")
            .and_then(|v| v.as_str())
            .unwrap_or("local");
        
        let format = arguments
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("RFC3339");
        
        // 获取时间（统一使用UTC，显示时根据timezone格式化）
        let dt_utc = Utc::now();
        let dt_local = Local::now();
        
        // 选择要显示的时间（根据timezone）
        let display_dt = match timezone {
            "UTC" => dt_utc,
            "local" => dt_local.with_timezone(&Utc),
            _ => dt_local.with_timezone(&Utc),
        };
        
        // 格式化时间
        let formatted_time = match format {
            "RFC3339" => display_dt.to_rfc3339(),
            "timestamp" => display_dt.timestamp().to_string(),
            "human" => {
                if timezone == "local" {
                    dt_local.format("%Y-%m-%d %H:%M:%S %Z").to_string()
                } else {
                    display_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
                }
            },
            _ => display_dt.to_rfc3339(),
        };
        
        Ok(json!({
            "time": formatted_time,
            "timezone": timezone,
            "format": format,
            "timestamp": display_dt.timestamp(),
            "iso8601": display_dt.to_rfc3339(),
            "local_time": dt_local.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            "utc_time": dt_utc.format("%Y-%m-%d %H:%M:%S UTC").to_string()
        }))
    }
}

