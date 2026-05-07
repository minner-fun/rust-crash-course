use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    code: i32,
    msg: String,
    data: Option<ExportTaskPage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportTaskPage {
    total_item: u32,
    page_index: u32,
    page_size: u32,
    item_list: Vec<ExportTask>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportTask {
    id: String,
    task_type_name: Option<String>,
    export_pin: Option<String>,
    task_status: i32,
    create_date: i64,
    task_data: TaskData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TaskData {
    export_file_pin: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
}

pub fn print_export_tasks(response_body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response: ApiResponse = serde_json::from_str(response_body)?;

    if response.code != 200 {
        return Err(format!("接口返回异常: code={}, msg={}", response.code, response.msg).into());
    }

    let Some(data) = response.data else {
        println!("接口返回成功，但 data 为空");
        return Ok(());
    };

    println!(
        "导出任务：第 {} 页，每页 {} 条，共 {} 条",
        data.page_index, data.page_size, data.total_item
    );
    println!(
        "{:<14} {:<8} {:<14} {:<24} {:<24} {:<23} {}",
        "任务ID",
        "状态",
        "任务类型",
        "开始时间",
        "结束时间",
        "创建时间",
        "店铺"
    );

    for task in data.item_list {
        let shop_name = task
            .export_pin
            .as_deref()
            .or(task.task_data.export_file_pin.as_deref())
            .unwrap_or("-");
        let task_type = task.task_type_name.as_deref().unwrap_or("-");
        let start_time = task.task_data.start_time.as_deref().unwrap_or("-");
        let end_time = task.task_data.end_time.as_deref().unwrap_or("-");

        println!(
            "{:<14} {:<8} {:<14} {:<24} {:<24} {:<23} {}",
            task.id,
            task_status_text(task.task_status),
            task_type,
            start_time,
            end_time,
            format_timestamp_millis(task.create_date),
            shop_name
        );
    }

    Ok(())
}

fn task_status_text(status: i32) -> &'static str {
    match status {
        0 => "待处理",
        1 => "处理中",
        2 => "已完成",
        3 => "失败",
        _ => "未知",
    }
}

fn format_timestamp_millis(timestamp: i64) -> String {
    chrono_like_format(timestamp).unwrap_or_else(|| timestamp.to_string())
}

fn chrono_like_format(timestamp: i64) -> Option<String> {
    use std::time::{Duration, UNIX_EPOCH};

    let millis = u64::try_from(timestamp).ok()?;
    let time = UNIX_EPOCH
        .checked_add(Duration::from_millis(millis))?
        .checked_add(Duration::from_secs(8 * 3_600))?;
    let seconds = time.duration_since(UNIX_EPOCH).ok()?.as_secs();
    let days = seconds / 86_400;
    let seconds_of_day = seconds % 86_400;

    let (year, month, day) = civil_from_days(i64::try_from(days).ok()?);
    let hour = seconds_of_day / 3_600;
    let minute = seconds_of_day % 3_600 / 60;
    let second = seconds_of_day % 60;

    Some(format!(
        "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02} CST"
    ))
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };

    (year as i32, m as u32, d as u32)
}
