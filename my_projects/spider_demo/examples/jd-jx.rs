extern crate reqwest;
use reqwest::header;
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

fn print_export_tasks(response_body: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        "任务ID", "状态", "任务类型", "开始时间", "结束时间", "创建时间", "店铺"
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert(
        "accept-language",
        "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7".parse().unwrap(),
    );
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert(
        "content-type",
        "application/json;charset=UTF-8".parse().unwrap(),
    );
    headers.insert("dsm-eid", "jdd03FB23UCWA5QALDJNH4OCKQJALXH5IMWPQ2HQEUFU4NI2SWN6DBWH4NFVEV7A5ZHK7KAP4HE6E4E555V3POJ6TAMB4LUAAAAM57NMZ7CIAAAAACGO7K7UFKH4YRQX".parse().unwrap());
    headers.insert("dsm-lang", "zh-CN".parse().unwrap());
    headers.insert("dsm-platform", "pc".parse().unwrap());
    headers.insert("dsm-site", "".parse().unwrap());
    headers.insert(
        "dsm-trace-id",
        "e214bf97-6afe-4131-be79-7965f447f2aa".parse().unwrap(),
    );
    headers.insert("h5st", "20260506135317605;pp5ip5pyziiabjv0;0248a;tk03w7a1b1b1f18n03EBSCmqX65vK0bJCv6najpW1NNWUqPvmp_7dI-A0uLjaJuRUihhDM42-x8d6lMB0G7GTCbwS_ec;ea4d23b270942d04c7641b66255d06ca;5.3;1778046792605;q3EpJrIOBiVTCGFNyqUe7GVe7qEjLDIj7SFjLrJp-TYfLDIj9e1TJrpjh7Jj7fYS4fIf7nYS6f4eFiVT2jod3nlSGmFS6bFfHS4fHSFjLDIj7SnQEiVS0ipjLDrgJf4TIWoSyXFS4bISzXVS2bVd4nIeFOYf6jlS2TVSHO4TJrJdJfUT1yVTIipjLDrgJnYgJrJdJbYOJipjLDrgJfIg4zZe1uWS-GFSMWoRJrJdJTEjLrJp-j5QR_VZgG3fF6VT3ipjxjZQ8aFQKiEjLrJp-jJf4zVRUipjxjJS7ipjLDrguqpjhfojxjZf6XETJrpjLrJp-bYfLDIj7nYOJipjLrpjh7pd_rJdJjYf2iFjLrpjLDrgJHXXjO1O9WVe4r0RHeEjLDIj_ulS9mFPJrpjLrJp-rojxjpd2iFjLrpjLDrg7rJdJPYOJipjLrpjh7pe6rJdJTYOJipjLrpjh7pfLDIj2XETJrpjLrJp-rojxjpe2iFjLrpjLDrg0bojxj5f2iFjLrpjLDrgJXIg6zpfJrJdJnYOJipjLrpjh7pfLDIjAOEjLrpjLDrg2rJdJfkQJrpjLrJp-rojxjpQJrpjLrJp-rojxjpS0ipjLrpjh-kjxjpS9WlOzWFjLrJp-3kjLDLjUBQy7JjxUNCy0BQyGlTyHBDyBJgyudjxDNQys9wyJrJdJjoPJrpjLrJpwqJdJrkPJrpjh7Jj3ToNL-oe1zVRUq5d7zpf6rpWdq5P0ulS9G1WJrJdJnVO4ipjLD7N;ebaf8a60fceca27ef967ce2c46a02591;pjLO-HkS8ilgHGnP7mlgCqUT".parse().unwrap());
    headers.insert("origin", "https://shop.jd.com".parse().unwrap());
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert("priority", "u=1, i".parse().unwrap());
    headers.insert(
        "referer",
        "https://shop.jd.com/jdm/trade/tools/export/ExprotList?_JDMOMID_=1568"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "sec-ch-ua",
        "\"Google Chrome\";v=\"147\", \"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"147\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
    headers.insert(header::COOKIE, "shshshfpa=3b971904-2852-a495-c649-adbad25fe779-1747211178; shshshfpx=3b971904-2852-a495-c649-adbad25fe779-1747211178; __jdu=17528296869521021399496; corpBehavior=1; joyya=1772000797.1772000798.35.1ej9788; shshshfpb=BApXWagVykPlAYbcU2hVKIXSAxM1pmVajBgVgFld79xJ1ItZfQtGGlkzv2C6sYdRydbOSheWPsqFbJes9vqUH7dsqZVvmrzLFmrMP; _jdjr_qy_sid=ditrM2QrdEF1cTcycFExV1luZTIrSXpib0RxeGhSaVFGdVRqRGVtWHZRMlZtY1ZXZGRoTU5BPT0=; qid_uid=cafb3689-76dd-4d55-9f45-c8c0f1bdee65; qid_fs=1778032755685; qid_ls=1778032755685; qid_ts=1778032755694; qid_vis=1; qyjr_U=K3ZZeWNtd1JpdlUxUTlOQ3pQaXoxZz09; qyjr_P=SmlBdUducTF0YXdoNENGb29NaUxsWm5DbDkzSFE4SlI=; qyjr_user=Iq/N5XXDHelc2B6O1MvG4793g2zgnB+50sOM684WVZrIfXlb+g8UQU/FdqTP9D3R; 3AB9D23F7A4B3C9B=FB23UCWA5QALDJNH4OCKQJALXH5IMWPQ2HQEUFU4NI2SWN6DBWH4NFVEV7A5ZHK7KAP4HE6E4E555V3POJ6TAMB4LU; qid_evord=98; __jdv=27966078|direct|-|none|-|1778035257990; language=zh_CN; dsm-lang=zh_CN; TrackID=1CyPx5LoLaBUa2ijGxOBU9WA_48nmHWomTF7xEUgyFvRoZ87e9X14PhXL5pDKggGdagWvTACjKP7rVzVr2BN-suos3nGX5tORuABQKWRNnEFOCb7M62q75X6Qmt6ZABvR; light_key=AASBKE7rOxgWQziEhC_QY6yaUYNXuEHUZq_mqNnw53UJCHD_lVwCX_3akAjW4Iqg6sPCWoJG-PnOP3iv6whUWG0pO3TkSA; pinId=3yBJMUdKx8JlzLATT9J9UWbW-thCI8KjCJMazSs1oVQ; pin=%E7%BB%BF%E5%B7%A8%E8%83%BD%E4%B8%AA%E6%8A%A4%E5%81%A5%E5%BA%B7%E6%97%97%E8%88%B0%E5%BA%97; unick=tp7ertjh38q95i; ceshi3.com=000; _tp=Ab9lXx15HqFyHzN1WbKxhcSXkQiNG%2B6z1kczA5fgnGBpSQGlth3vL4O7ON0NmQGoHOkV6ED9K0tYYnKWDKalCpx%2BDgTOklZft02EYe123Rg0DJZOMBr%2FX3wZJ7v2cZ8O; _pst=%E7%BB%BF%E5%B7%A8%E8%83%BD%E4%B8%AA%E6%8A%A4%E5%81%A5%E5%BA%B7%E6%97%97%E8%88%B0%E5%BA%97; __jdc=27966078; _s_vender_=7CG6RFKUPSA2XWZTJZX2HYZ4E3UK3H2LQFKUTN4AYIWQHJRU2WYSNIY4JAJZD6JVACOJV5ABNFYXLOMPTACKPRZ7EJ7YX5S6LJQGV4Q5GCKSIRRIPC7CESXL6ZBV6WH6JYGJ4AHQKTZI3U3UWCWPBFJ2BSH66Q3FNB655Q47QVRC3KYZJK6GM6QE5ASQKHWWHB47ILPKU5VDCG2KZEAVUGMJRBK45DQRE6ECRJETG6V5B5CB3ULPXPZPFKCQGMNK6TSJIKQRFJEROPM6AJ5NBXFIH373X4UYHQQEJXAFMUDGIE63TPK7UPUHHHZ52ABTV4CHA4M2SF3RNZP3M4OZAEP3HRTOIG4NWYMVRXEU7IWE7CHB4O4XTNJHNY7K4V7XSTVHVG3GBB6KV677JQTHYGJR2Q4YAXQAHPBGZ35DIOEUTBZGJKSQ; __USE_NEW_PAGEFRAME__=true; __USE_NEW_PAGEFRAME_VERSION__=v11; __jda=27966078.17528296869521021399496.1752829687.1778035258.1778038452.10; 3AB9D23F7A4B3CSS=jdd03FB23UCWA5QALDJNH4OCKQJALXH5IMWPQ2HQEUFU4NI2SWN6DBWH4NFVEV7A5ZHK7KAP4HE6E4E555V3POJ6TAMB4LUAAAAM57NMZ7CIAAAAACGO7K7UFKH4YRQX; thor=4431E6981817902F4B36093BD9706C14BC7CEA5D454E832F099E0AC4F38409595F85AB04A342C0C4E2D9608FFBBF3140E13F3230DEED7E7AC8BD75458255AC7DEE8D0A1418547C75D489869798E6A040EF941A82399A9FE394050EC635A539FB7DE5395594CEA8E567EE0DADF5F9C167EABEE59C5F9B5C83A83FA155149B2023D2597F200E63A8E90027ECAF14B1592F; _base_=YKH2KDFHMOZBLCUV7NSRBWQUJPBI7JIMU5R3EFJ5UDHJ5LCU7R2NILKK5UJ6GLA2RGYT464UKXAI4Z6HPCTN4UQM3WHVQ4ENFP57OC3MTMGEOPLXNWBEPYGYG44HJC74TCNE6YVKRXISVWSCJHKC27CZSXWDA4AEL7ZLXI2SG4SOQWCP5WPWO6EFS7HEHMRWVKBRVHB33TFD4OVVZ2SUTIRYRHIAPD6N4V4LDONPWRQTNI5OQB4F5JNG2QXIWWOVYQ7OHSQM4ZUUB4HYL4A2TQWTOZU63NKLXTL5WAC5MH7WCDNBGIY5H6X7AYQ7MSBX4XRAEBH6YHM5OXZPIF5D6AMSEWLAI2XBNJFMOQY; flash=3_rOBedvPq2Ku7Kpvv6h2Sm_dGVECzU5e62TltKuNTg4O7Ct5exMrBAnXImouasMNw09GPAI9fW7NtcRbwA33hyOu9VN2nQ6KNnurfAzhUMJ3omhCg7zlvLLmzVI8l5kEBQwW0F3C0Izwt7k6fFIBEbvRRiVlWX9iEvzE-Qzcs1W9cCfppo_zSVUO79XSio_u8XM64lFSo; _s_base_=7CG6RFKUPSA2XWZTJZX2HYZ4E3UK3H2LQFKUTN4AYIWQHJRU2WYUJWHGNXNBO2OUC7NLCGW546QEO74XDAI4T6T7MX2RA7Q5MOLADDIJUC3GFPOOWHPJHNN64GWMELKNJUOCLDHKOTG5TXREKMPAMVN6KYKOQK7Q7KJ6G2MU5J5JWZQIPSVPX72MEZ6BSMOU4GVLR2HP7ONCHI2DRFEYOJSKUU".parse().unwrap());

    let url = "https://sff.jd.com/api?v=1.0&appId=CQLEJWPYPFOVQBC8UFLQ&api=dsm.order.export.exportCenterService.queryExportTaskInfo";
    // let client = reqwest::blocking::Client::builder()
    //     .redirect(reqwest::redirect::Policy::none())
    //     .build()
    //     .unwrap();
    // let res = client.post("https://sff.jd.com/api?v=1.0&appId=CQLEJWPYPFOVQBC8UFLQ&api=dsm.order.export.exportCenterService.queryExportTaskInfo")
    //     .headers(headers)
    //     .body("{\"exportParam\":{\"exportTaskType\":0,\"page\":1,\"pageSize\":10}}")
    //     .send()?
    //     .text()?;

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .body("{\"exportParam\":{\"exportTaskType\":0,\"page\":1,\"pageSize\":10}}")
        .send()?
        .text()?;

    print_export_tasks(&res)?;

    Ok(())
}
