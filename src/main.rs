use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use clap::Parser;
use log::info;
use polars::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::{from_str, Value};
use std::fs::OpenOptions;

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

/// A subcommand for report
#[derive(Parser)]
struct ReportOpts {
    /// the miner to generate report
    #[clap(
        short = 'm',
        long = "miner",
        default_value = "ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj"
    )]
    miner: String,
}

#[derive(Parser)]
enum SubCommand {
    /// sync data from chain and show today's statistics info
    #[clap(name = "top")]
    Top(TopOpts),
    /// show some day's statistics info
    #[clap(name = "history")]
    History(HistoryOpts),
    /// generate report about one miner
    #[clap(name = "report")]
    Report(ReportOpts),
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
        }
        SubCommand::History(opts) => {
            run_history(opts);
        }
        SubCommand::Report(opts) => {
            run_report(opts);
        }
    }
}

fn init_blocks_parquet(file_path: &str) {
    // create parquet file and set init values
    // "Blockno,Transactions,UnixTimestamp,Reward,Miner,Date\n")
    // 15001369,1,1735689592485,562.42421899,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2024-12-31 23:59:52
    let mut df = df! {
        "Blockno" => &[15001369u64],
        "Transactions" => &[1u32],
        "UnixTimestamp" => &[1735689592485u64],
        "Reward" => &[562.42421899_f64],
        "Miner" => &["ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj".to_string()],
        "Date" => &[NaiveDateTime::parse_from_str("2024-12-31 23:59:52", "%Y-%m-%d %H:%M:%S").unwrap()]
    }.unwrap();
    let file = std::fs::File::create(file_path).unwrap();
    ParquetWriter::new(file).finish(&mut df).unwrap();
}

fn get_latest_block(df: &DataFrame) -> u64 {
    let lf_max = df.clone().lazy().max().collect().unwrap();
    let blockno_data = lf_max.column("Blockno").unwrap();
    let max_blockno = blockno_data.get(0).unwrap();
    if let AnyValue::UInt64(max_blockno) = max_blockno {
        max_blockno
    } else {
        panic!("max blockno is not Uint64");
    }
}

async fn get_tip_info() -> (u64, f64) {
    // curl 'https://mainnet-api.explorer.nervos.org/api/v1/statistics' -H 'accept: application/vnd.api+json' -H 'content-type: application/vnd.api+json'
    let url = "https://mainnet-api.explorer.nervos.org/api/v1/statistics";
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let content_type_value = HeaderValue::from_str("application/vnd.api+json").unwrap();
    headers.insert("Content-Type", content_type_value.clone());
    headers.insert("Accept", content_type_value);

    let response = client.get(url).headers(headers).send().await.unwrap();
    let body = response.text().await.unwrap();

    let json_value: Value = from_str(&body).unwrap();
    let tip_block_str = json_value
        .get("data")
        .unwrap()
        .get("attributes")
        .unwrap()
        .get("tip_block_number")
        .unwrap()
        .as_str()
        .unwrap();
    let hashrate = json_value
        .get("data")
        .unwrap()
        .get("attributes")
        .unwrap()
        .get("hash_rate")
        .unwrap()
        .as_str()
        .unwrap();
    let tip_block = tip_block_str.parse::<u64>().unwrap();
    let hashrate = hashrate.parse::<f64>().unwrap();

    (tip_block, hashrate)
}

fn init_hashrate_parquet(file_path: &str) {
    // create parquet file and set init values
    // "Timestamp,HashRate\n")
    // 1735574400,442388595130533.780735
    let mut df = df! {
        "Timestamp" => &[1735574400i64],
        "HashRate" => &[442_388_595_130_533.8_f64]
    }
    .unwrap();
    let file = std::fs::File::create(file_path).unwrap();
    ParquetWriter::new(file).finish(&mut df).unwrap();
}

fn get_latest_timestamp_hashrate(df: &DataFrame) -> i64 {
    let lf_max = df.clone().lazy().max().collect().unwrap();
    let timestamp_data = lf_max.column("Timestamp").unwrap();
    let max_timestamp = timestamp_data.get(0).unwrap();
    if let AnyValue::Int64(max_timestamp) = max_timestamp {
        max_timestamp
    } else {
        panic!("max date is not Int64");
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
        init_blocks_parquet(file_path);
    }

    // read parquet file
    let mut file = std::fs::File::open(file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    // get latest block
    latest_block = std::cmp::max(latest_block, get_latest_block(&df));
    info!("latest block is: {}", latest_block);

    // query from tip block
    let (tip_block, hashrate) = get_tip_info().await;
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

        let response = client.get(url).headers(headers).send().await.unwrap();
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
    }
    .unwrap();

    df_new
        .sort_in_place(["Blockno"], Default::default())
        .unwrap();

    let df = df.vstack(&df_new).unwrap();

    // split DataFrame by date
    // process today's data
    let mut latest_df = df
        .clone()
        .lazy()
        .filter(col("Date").is_between(
            lit(current_date.and_hms_opt(0, 0, 0).unwrap()),
            lit(current_date.and_hms_opt(23, 59, 59).unwrap()),
            ClosedInterval::Both,
        ))
        .collect()
        .unwrap();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    ParquetWriter::new(file).finish(&mut latest_df).unwrap();

    // process history data
    let mut tmp_date = current_date - Duration::days(1);
    loop {
        let mut history_df = df
            .clone()
            .lazy()
            .filter(col("Date").is_between(
                lit(tmp_date.and_hms_opt(0, 0, 0).unwrap()),
                lit(tmp_date.and_hms_opt(23, 59, 59).unwrap()),
                ClosedInterval::Both,
            ))
            .collect()
            .unwrap();

        // no data for date then stop
        if history_df.height() == 0 {
            break;
        }

        // write data to parquet file
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("./data/ckb-blocks-{}.parquet", tmp_date))
            .unwrap();
        ParquetWriter::new(file).finish(&mut history_df).unwrap();

        tmp_date -= Duration::days(1);
    }

    // sync history hashrate
    let hash_rate_file_path = "./data/ckb-hashrate.parquet";
    let file_exist = std::path::Path::new(&hash_rate_file_path).exists();
    if !file_exist {
        init_hashrate_parquet(hash_rate_file_path);
    }

    let mut file = std::fs::File::open(hash_rate_file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    // get latest date of hashrate
    let latest_timestamp = get_latest_timestamp_hashrate(&df);
    let latest_date = DateTime::from_timestamp(latest_timestamp, 0)
        .unwrap()
        .date_naive();

    // timestamp of hashrate is lastday of 16:00
    // for example,  hashrate of 2025-01-06, it's timestamp is 2025-01-05 16:00:00
    if latest_date < current_date - Duration::days(2) {
        // sync history hash rate
        let url = "https://mainnet-api.explorer.nervos.org/api/v1/daily_statistics/avg_hash_rate";
        let client = Client::new();
        let mut headers = HeaderMap::new();
        let content_type_value = HeaderValue::from_str("application/vnd.api+json").unwrap();
        headers.insert("Content-Type", content_type_value.clone());
        headers.insert("Accept", content_type_value);

        let response = client.get(url).headers(headers).send().await.unwrap();
        let body = response.text().await.unwrap();

        let json_value: Value = from_str(&body).unwrap();
        let json_data = json_value.get("data").unwrap().as_array().unwrap();

        let mut timestamp_data: Vec<i64> = Vec::new();
        let mut hashrate_data: Vec<f64> = Vec::new();

        for data in json_data.iter() {
            let created_at_unixtimestamp = data
                .get("attributes")
                .unwrap()
                .get("created_at_unixtimestamp")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            // skip data before latest_timestamp
            if created_at_unixtimestamp < latest_timestamp {
                continue;
            }
            let avg_hash_rate = data
                .get("attributes")
                .unwrap()
                .get("avg_hash_rate")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<f64>()
                .unwrap();
            timestamp_data.push(created_at_unixtimestamp);
            hashrate_data.push(avg_hash_rate);
        }

        let df_new = df! {
            "Timestamp" => &timestamp_data,
            "HashRate" => &hashrate_data
        }
        .unwrap();

        let mut df = df.vstack(&df_new).unwrap();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(hash_rate_file_path)
            .unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();
    }

    // show today's statistics info
    let percent = col("Count") * 100.0f64.into() / col("Total_Count");
    let user_hash_rate = col("Percent") * hashrate.into() / 100.0f64.into();
    let mut result = latest_df
        .clone()
        .lazy()
        .group_by([(col("Miner"))])
        .agg([
            col("Blockno").count().alias("Count"),
            col("Reward").sum().alias("User_Reward"),
        ])
        .with_columns([col("Count").sum().alias("Total_Count")])
        .with_columns([percent.alias("Percent")])
        .with_columns([user_hash_rate.alias("User_Hash_Rate")])
        .collect()
        .unwrap();

    result.sort_in_place(["Count"], Default::default()).unwrap();

    println!("{}", result);
}

fn get_hashrate_by_date(date: NaiveDate) -> f64 {
    // timestamp of hashrate is lastday of 16:00
    // for example,  hashrate of 2025-01-06, it's timestamp is 2025-01-05 16:00:00
    let s_timestamp = date.and_hms_opt(16, 0, 0).unwrap().and_utc().timestamp() - 3600 * 24;

    let hash_rate_file_path = "./data/ckb-hashrate.parquet";
    let mut file = std::fs::File::open(hash_rate_file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    let lf = df
        .clone()
        .lazy()
        .filter(col("Timestamp").eq(lit(s_timestamp)))
        .collect()
        .unwrap();
    let hashrate_data = lf.column("HashRate").unwrap();
    let hashrate = hashrate_data.get(0).unwrap();
    if let AnyValue::Float64(hashrate) = hashrate {
        hashrate
    } else {
        panic!("hashrate is not Float64");
    }
}

#[tokio::main]
async fn run_history(opts: HistoryOpts) {
    let data_number = opts.date;
    let s_date = NaiveDate::from_ymd_opt(
        (data_number / 10000) as i32,
        ((data_number % 10000) / 100) as u32,
        (data_number % 100) as u32,
    )
    .unwrap();
    println!("show history info for date: {}", s_date);

    let file_path = format!("./data/ckb-blocks-{}.parquet", s_date);
    let file_exist = std::path::Path::new(&file_path).exists();
    if !file_exist {
        info!("no data for date: {}", s_date);
        return;
    }

    // find history hashrate
    let hash_rate = get_hashrate_by_date(s_date);

    // read parquet file
    let mut file = std::fs::File::open(file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    let percent = col("Count") * 100.0f64.into() / col("Total_Count");
    let user_hash_rate = col("Percent") * hash_rate.into() / 100.0f64.into();
    let mut result = df
        .clone()
        .lazy()
        .group_by([(col("Miner"))])
        .agg([
            col("Blockno").count().alias("Count"),
            col("Reward").sum().alias("User_Reward"),
        ])
        .with_columns([col("Count").sum().alias("Total_Count")])
        .with_columns([percent.alias("Percent")])
        .with_columns([user_hash_rate.alias("User_Hash_Rate")])
        .collect()
        .unwrap();

    result.sort_in_place(["Count"], Default::default()).unwrap();

    println!("{}", result);
}

#[tokio::main]
async fn run_report(opts: ReportOpts) {
    let miner = &opts.miner;
    info!("generate report for miner: {}", &miner);

    let current_time = Utc::now();
    info!("Current time: {}", current_time);
    let current_date = current_time.date_naive();

    // get tip block and hashrate
    let (_tip_block, hashrate) = get_tip_info().await;

    // get today's data
    let file_path = "./data/ckb-blocks.parquet";
    let file_exist = std::path::Path::new(&file_path).exists();
    if !file_exist {
        info!("no data for today");
        return;
    }
    let mut file = std::fs::File::open(file_path).unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    let percent = col("Count") * 100.0f64.into() / col("Total_Count");
    let user_hash_rate = col("Percent") * hashrate.into() / 100.0f64.into();
    let mut ret_df = df
        .clone()
        .lazy()
        .group_by([(col("Miner"))])
        .agg([
            col("Blockno").count().alias("Count"),
            col("Reward").sum().alias("User_Reward"),
        ])
        .with_columns([col("Count").sum().alias("Total_Count")])
        .with_columns([percent.alias("Percent")])
        .with_columns([user_hash_rate.alias("User_Hash_Rate")])
        .filter(col("Miner").eq(lit(miner.clone())))
        .with_columns([lit(current_time.naive_utc()).alias("Time")])
        .collect()
        .unwrap();

    // get history data
    let mut tmp_date = current_date - Duration::days(1);
    loop {
        let file_path = format!("./data/ckb-blocks-{}.parquet", tmp_date);
        let file_exist = std::path::Path::new(&file_path).exists();
        if !file_exist {
            break;
        }

        // get hash rate by date
        let hash_rate = get_hashrate_by_date(tmp_date);

        // read parquet file
        let mut file = std::fs::File::open(file_path).unwrap();
        let df = ParquetReader::new(&mut file).finish().unwrap();

        let percent = col("Count") * 100.0f64.into() / col("Total_Count");
        let user_hash_rate = col("Percent") * hash_rate.into() / 100.0f64.into();
        let data_df = df
            .clone()
            .lazy()
            .group_by([(col("Miner"))])
            .agg([
                col("Blockno").count().alias("Count"),
                col("Reward").sum().alias("User_Reward"),
            ])
            .with_columns([col("Count").sum().alias("Total_Count")])
            .with_columns([percent.alias("Percent")])
            .with_columns([user_hash_rate.alias("User_Hash_Rate")])
            .filter(col("Miner").eq(lit(miner.clone())))
            .with_columns([lit(tmp_date.and_hms_opt(23, 59, 59).unwrap()).alias("Time")])
            .collect()
            .unwrap();

        ret_df = ret_df.vstack(&data_df).unwrap();

        tmp_date -= Duration::days(1);
    }

    println!("{}", ret_df);

    // write report to csv file
    let file_path = format!("./data/ckb-report-{}.csv", miner);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    CsvWriter::new(&mut file).finish(&mut ret_df).unwrap();
}
