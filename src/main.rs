use clap::Parser;
use log::info;
use chrono::{Utc, NaiveDate, NaiveDateTime, Duration, DateTime};
use polars::prelude::*;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, from_str};
use std::fs::OpenOptions;
use std::io::Write;

/// A subcommand for top
#[derive(Parser)]
struct TopOpts {
    /// Chain config path
    #[clap(short = 'b', long = "begin-block", default_value = "15001369")]
    begin_block: u64,
}

/// A subcommand for history
#[derive(Parser)]
struct HistoryOpts {
    /// the date to show history info
    #[clap(short = 'd', long = "date", default_value = "20250101")]
    date: u64,
}

#[derive(Parser)]
enum SubCommand {
    /// sync data from chain and show today's statistics info
    #[clap(name = "top")]
    Top(TopOpts),
    /// show some day's statistics info
    #[clap(name = "history")]
    History(HistoryOpts),

}

pub fn clap_about() -> String {
    let name = env!("CARGO_PKG_NAME").to_string();
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    name + " " + version + "\n" + authors
}

#[derive(Parser)]
#[clap(version, about = clap_about())]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "full");
    ::std::env::set_var("RUST_LOG", "info");
    ::std::env::set_var("POLARS_FMT_STR_LEN", "256");

    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Top(opts) => {
            run_top(opts);
        },
        SubCommand::History(opts) => {
            run_history(opts);
        }

    }
}

#[tokio::main]
async fn run_top(opts: TopOpts) {
    let current_time = Utc::now();
    info!("Current time: {}", current_time);
    let current_date = current_time.date_naive();

    let mut latest_block: u64 = opts.begin_block;

    let file_path = "./data/ckb-blocks.parquet";
    let file_exist = std::path::Path::new(&file_path).exists();
    if !file_exist {
        // create parquet file and set init values
        // "Blockno,Transactions,UnixTimestamp,Reward,Miner,Date\n")
        // 15001369,1,1735689592485,562.42421899,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2024-12-31 23:59:52
        let mut df = df! {
            "Blockno" => &[15001369u64],
            "Transactions" => &[1u32],
            "UnixTimestamp" => &[1735689592485u64],
            "Reward" => &[562.42421899 as f64],
            "Miner" => &["ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj".to_string()],
            "Date" => &[NaiveDateTime::parse_from_str("2024-12-31 23:59:52", "%Y-%m-%d %H:%M:%S").unwrap()]
        }.unwrap();
        let file = std::fs::File::create(file_path).unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();
    }

    // read parquet file
    let mut file = std::fs::File::open(file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    // get latest block
    let lf_max = df.clone().lazy().max().collect().unwrap();
    let blockno_data = lf_max.column("Blockno").unwrap();
    let max_blockno = blockno_data.get(0).unwrap();
    if let AnyValue::UInt64(max_blockno) = max_blockno {
        latest_block = std::cmp::max(max_blockno, latest_block);
    } else {
        panic!("max blockno is not Uint64");
    }
    info!("latest block is: {}", latest_block);

    // query from tip block
    // curl 'https://mainnet-api.explorer.nervos.org/api/v1/statistics' -H 'accept: application/vnd.api+json' -H 'content-type: application/vnd.api+json'
    let url = "https://mainnet-api.explorer.nervos.org/api/v1/statistics";
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let content_type_value = HeaderValue::from_str("application/vnd.api+json").unwrap();
    headers.insert("Content-Type", content_type_value.clone());
    headers.insert("Accept", content_type_value);

    let response = client
       .get(url)
       .headers(headers)
       .send()
       .await
       .unwrap();
    let body = response.text().await.unwrap();

    let json_value: Value = from_str(&body).unwrap();
    let tip_block_str = json_value.get("data").unwrap().get("attributes").unwrap().get("tip_block_number").unwrap().as_str().unwrap();
    let hashrate = json_value.get("data").unwrap().get("attributes").unwrap().get("hash_rate").unwrap().as_str().unwrap();
    let tip_block = tip_block_str.parse::<u64>().unwrap();

    info!("tip_block: {}, current hashrate {}", tip_block, hashrate);

    // sync blocks
    let mut begin = latest_block + 1;
    let mut end = std::cmp::min(latest_block + 3000, tip_block);

    let mut blockno_data: Vec<u64> = Vec::new();
    let mut transactions_data: Vec<u32> = Vec::new();
    let mut unix_timestamp_data: Vec<u64> = Vec::new();
    let mut reward_data: Vec<f64> = Vec::new();
    let mut miner_data: Vec<String> = Vec::new();
    let mut date_data: Vec<NaiveDateTime> = Vec::new();

    loop {
        info!("sync from {} to {}", begin, end);
        // sync blocks
        let url = format!("https://mainnet-api.explorer.nervos.org/api/v1/blocks/download_csv?start_number={}&end_number={}",begin, end);
        let client = Client::new();
        let mut headers = HeaderMap::new();
        let content_type_value = HeaderValue::from_str("application/vnd.api+json").unwrap();
        headers.insert("Content-Type", content_type_value.clone());
        headers.insert("Accept", content_type_value);
    
        let response = client
           .get(url)
           .headers(headers)
           .send()
           .await
           .unwrap();
        let body = response.text().await.unwrap();
        let lines: Vec<&str> = body.split('\n').collect();
        info!("got lines: {}", lines.len());

        for line in lines[1..].iter() {
            if line.is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split(',').collect();
            let blockno = fields[0].parse::<u64>().unwrap();
            let transactions = fields[1].parse::<u32>().unwrap();
            let unix_timestamp = fields[2].parse::<u64>().unwrap();
            let reward = fields[3].parse::<f64>().unwrap();
            let miner = fields[4].to_string();
            let date = NaiveDateTime::parse_from_str(fields[5], "%Y-%m-%d %H:%M:%S").unwrap();
            blockno_data.push(blockno);
            transactions_data.push(transactions);
            unix_timestamp_data.push(unix_timestamp);
            reward_data.push(reward);
            miner_data.push(miner);
            date_data.push(date);
        }

        if end == tip_block {
            break;
        }
        begin = end + 1;
        end = std::cmp::min(end + 3000, tip_block);
        
        // sleep 5s
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    info!("sync done!");

    let mut df_new = df! {
        "Blockno" => blockno_data,
        "Transactions" => transactions_data,
        "UnixTimestamp" => unix_timestamp_data,
        "Reward" => reward_data,
        "Miner" => miner_data,
        "Date" => date_data
    }.unwrap();

    df_new.sort_in_place(["Blockno"], Default::default()).unwrap();

    let df = df.vstack(&df_new).unwrap();

    // split dataframe by date
    let mut latest_df = df
    .clone()
    .lazy()
    .filter(
        col("Date")
            .is_between(
                lit(current_date.and_hms_opt(0, 0, 0).unwrap()),
                lit(current_date.and_hms_opt(23, 59, 59).unwrap()),
                ClosedInterval::Both,
            )
    )
    .collect().unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    ParquetWriter::new(file).finish(&mut latest_df).unwrap();

    let mut tmp_date = current_date - Duration::days(1);
    loop {
        let mut history_df = df
        .clone()
        .lazy()
        .filter(
            col("Date")
                .is_between(
                    lit(tmp_date.and_hms_opt(0, 0, 0).unwrap()),
                    lit(tmp_date.and_hms_opt(23, 59, 59).unwrap()),
                    ClosedInterval::Both,
                )
        )
        .collect().unwrap();

        if history_df.height() == 0 {
            break;
        }
    
        // write data to parquet file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("./data/ckb-blocks-{}.parquet", tmp_date))
            .unwrap();
        ParquetWriter::new(file).finish(&mut history_df).unwrap();

        tmp_date = tmp_date - Duration::days(1);
    }


    // sync history hashrate
    let hash_rate_file_path = "./data/ckb-hashrate.parquet";
    let file_exist = std::path::Path::new(&hash_rate_file_path).exists();
    if !file_exist {
        // create parquet file and set init values
        // "Timestamp,HashRate\n")
        // "created_at_unixtimestamp":"1735660800","avg_hash_rate":"452119131191160.156639
        let mut df = df! {
            "Timestamp" => &[1735660800i64],
            "HashRate" => &[452119131191160.156639f64]
        }.unwrap();
        let file = std::fs::File::create(hash_rate_file_path).unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();
    }

    let mut file = std::fs::File::open(hash_rate_file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    // get latest date of hashrate
    let mut lastest_timestamp = 1735747200i64;
    let lf_max = df.clone().lazy().max().collect().unwrap();
    let timestamp_data = lf_max.column("Timestamp").unwrap();
    let max_timestamp = timestamp_data.get(0).unwrap();
    if let AnyValue::Int64(max_timestamp) = max_timestamp {
        lastest_timestamp = max_timestamp;
    } else {
        panic!("max date is not Int64");
    }
    let lastest_date = DateTime::from_timestamp(lastest_timestamp, 0).unwrap().date_naive();

    if lastest_date < current_date - Duration::days(1) {
        // sync history hash rate
        let url = "https://mainnet-api.explorer.nervos.org/api/v1/daily_statistics/avg_hash_rate";
        let client = Client::new();
        let mut headers = HeaderMap::new();
        let content_type_value = HeaderValue::from_str("application/vnd.api+json").unwrap();
        headers.insert("Content-Type", content_type_value.clone());
        headers.insert("Accept", content_type_value);

        let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap();
        let body = response.text().await.unwrap();

        let json_value: Value = from_str(&body).unwrap();
        let json_data = json_value.get("data").unwrap().as_array().unwrap();

        let mut timestamp_data: Vec<i64> = Vec::new();
        let mut hashrate_data: Vec<f64> = Vec::new();

        for data in json_data.iter() {
            let created_at_unixtimestamp = data.get("attributes").unwrap().get("created_at_unixtimestamp").unwrap().as_str().unwrap().parse::<i64>().unwrap();
            if created_at_unixtimestamp <= lastest_timestamp {
                continue;
            }
            let avg_hash_rate = data.get("attributes").unwrap().get("avg_hash_rate").unwrap().as_str().unwrap().parse::<f64>().unwrap();
            timestamp_data.push(created_at_unixtimestamp);
            hashrate_data.push(avg_hash_rate);
        }

        let mut df_new = df! {
            "Timestamp" => &timestamp_data,
            "HashRate" => &hashrate_data
        }.unwrap();

        let mut df = df.vstack(&df_new).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(hash_rate_file_path)
            .unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();
    }

    // show today's statistics info
    let percent = col("Count") * 100.0f64.into() / col("Total_Count");
    let hash_rate = hashrate.parse::<f64>().unwrap();
    let user_hash_rate = col("Percent") * hash_rate.into() / 100.0f64.into();
    let mut result = latest_df
    .clone()
    .lazy()
    .group_by([(col("Miner"))])
    .agg([
        col("Blockno").count().alias("Count"),
        col("Reward").sum().alias("User_Reward")
    ])
    .with_columns([
        col("Count").sum().alias("Total_Count")
    ])
    .with_columns([
        percent.alias("Percent")
    ])
    .with_columns([
        user_hash_rate.alias("User_Hash_Rate")
    ])
    .collect().unwrap();

    result.sort_in_place(["Count"], Default::default()).unwrap();

    println!("{}", result);
}

#[tokio::main]
async fn run_history(opts: HistoryOpts) {
    let data_number = opts.date;
    let s_date = NaiveDate::from_ymd_opt((data_number /10000) as i32, ((data_number % 10000) / 100) as u32, (data_number % 100) as u32).unwrap();
    println!("show history info for date: {}", s_date);

    let file_path = format!("./data/ckb-blocks-{}.parquet", s_date);
    let file_exist = std::path::Path::new(&file_path).exists();
    if !file_exist {
        info!("no data for date: {}", s_date);
        return;
    }

    // find history hashrate
    let mut s_hash_rate = 444485748006408.689918f64;
    let s_timestamp = s_date.and_hms_opt(16, 0, 0).unwrap().and_utc().timestamp() - 3600 * 24;
    let hash_rate_file_path = "./data/ckb-hashrate.parquet";
    let mut file = std::fs::File::open(hash_rate_file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    let result = df.clone().lazy().filter(col("Timestamp").eq(lit(s_timestamp))).collect().unwrap();
    let hashrate_data = result.column("HashRate").unwrap();
    let hash_rate = hashrate_data.get(0).unwrap();
    if let AnyValue::Float64(hash_rate) = hash_rate {
        s_hash_rate = hash_rate;
    } else {
        panic!("hash rate is not Float64");
    }

    // read parquet file
    let mut file = std::fs::File::open(file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    let percent = col("Count") * 100.0f64.into() / col("Total_Count");
    let hash_rate = s_hash_rate;
    let user_hash_rate = col("Percent") * hash_rate.into() / 100.0f64.into();
    let mut result = df
    .clone()
    .lazy()
    .group_by([(col("Miner"))])
    .agg([
        col("Blockno").count().alias("Count"),
        col("Reward").sum().alias("User_Reward")
    ])
    .with_columns([
        col("Count").sum().alias("Total_Count")
    ])
    .with_columns([
        percent.alias("Percent")
    ])
    .with_columns([
        user_hash_rate.alias("User_Hash_Rate")
    ])
    .collect().unwrap();

    result.sort_in_place(["Count"], Default::default()).unwrap();

    println!("{}", result);
}
