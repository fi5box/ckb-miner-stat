# ckb-miner-stat
statistics info about ckb miners

## data source

all data from explorer.nervos.org

```
$ curl 'https://mainnet-api.explorer.nervos.org/api/v1/blocks/download_csv?start_number=15021000&end_number=15021010' -H 'accept: application/vnd.api+json' -H 'content-type: application/vnd.api+json'
Blockno,Transactions,UnixTimestamp,Reward(CKB),Miner,date(UTC)
15021010,1,1735869224432,793.37619725,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0259kv0x5jez6h0l5qr52k64weapapgyscsnmhn,2025-01-03 01:53:44
15021009,1,1735869208100,793.37619762,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq22xdj8q4jdql98qkdhnqzgrsk4nqyavdcratvu9,2025-01-03 01:53:28
15021008,1,1735869200131,793.37619799,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvgqh40v2g5ps3rann79tvw9gq6ewrft7gp909qk,2025-01-03 01:53:20
15021007,4,1735869199299,793.37619786,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0dn8gg6ag6uvkl0lr0xpyt0n99dsal47sm7mzyj,2025-01-03 01:53:19
15021006,4,1735869188452,793.37619823,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2025-01-03 01:53:08
15021005,4,1735869185901,793.37619786,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0dn8gg6ag6uvkl0lr0xpyt0n99dsal47sm7mzyj,2025-01-03 01:53:05
15021004,5,1735869172139,793.37619773,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvgqh40v2g5ps3rann79tvw9gq6ewrft7gp909qk,2025-01-03 01:52:52
15021003,2,1735869163968,793.3761981,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2025-01-03 01:52:43
15021002,3,1735869143766,793.37619797,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2025-01-03 01:52:23
15021001,7,1735869129298,793.37619834,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2025-01-03 01:52:09
15021000,1,1735869120016,793.37619872,ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2025-01-03 01:52:00
```

```
$ curl 'https://mainnet-api.explorer.nervos.org/api/v1/statistics' -H 'accept: application/vnd.api+json' -H 'content-type: application/vnd.api+json'
{
    "data": {
        "id": "1",
        "type": "index_statistic",
        "attributes": {
            "epoch_info": {
                "epoch_number": "11220",
                "epoch_length": "1705",
                "index": "644"
            },
            "tip_block_number": "15022536",
            "average_block_time": "8516.17",
            "current_epoch_difficulty": "3912039464560149031",
            "hash_rate": "467692630824115.730368",
            "estimated_epoch_time": "14261561.648559",
            "transactions_last_24hrs": "26649",
            "transactions_count_per_minute": "19",
            "reorg_started_at": null
        },
        "relationships": {

        }
    }
}
```



```
curl 'https://mainnet-api.explorer.nervos.org/api/v1/daily_statistics/avg_hash_rate' -H 'accept: application/vnd.api+json' -H 'content-type: application/vnd.api+json' 
{
    "data": [
        {
            "id": "1518",
            "type": "daily_statistic",
            "attributes": {
                "created_at_unixtimestamp": "1573833600",
                "avg_hash_rate": "73466099633.870151"
            }
        },
        ...
        {
            "id": "1533",
            "type": "daily_statistic",
            "attributes": {
                "created_at_unixtimestamp": "1575129600",
                "avg_hash_rate": "171709274179.29221"
            }
        }
    ]
}
```

## statistics info

### simple info
include date,  begin blockno and end blockno, total block count, total reward, total hashrate. 

### miner info
group block info by Miner, sum block count and reward.
calc percent of each miner, then calc hashrate of each miner by total hashrate.

### history

default from GMT 2025-01-01 00:00:00
begin blockno is 15001370


## data file

```
 ──────────┬──────────────┬───────────────┬────────────┬─────────────────────────────────┬─────────────────────────┐
│ Blockno  ┆ Transactions ┆ UnixTimestamp ┆ Reward     ┆ Miner                           ┆ Date                    │
│ ---      ┆ ---          ┆ ---           ┆ ---        ┆ ---                             ┆ ---                     │
│ u64      ┆ u32          ┆ u64           ┆ f64        ┆ str                             ┆ datetime[μs, UTC]       │
╞══════════╪══════════════╪═══════════════╪════════════╪═════════════════════════════════╪═════════════════════════╡
│ 15011060 ┆ 66           ┆ 1.7358e12     ┆ 797.132369 ┆ ckb1qzda0cr08m85hc8jlnfp3zer7x… ┆ 2025-01-01 23:59:44 UTC |
```


## useage

```
ckb-miner-stat 0.1.0
david <david@fi5box.com>

Usage: ckb-miner-stat <COMMAND>

Commands:
  top      sync data from chain and show today's statistics info
  history  show some day's statistics info
  report   generate report about one miner
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### top

```
$ ./target/debug/ckb-miner-stat top -h
sync data from chain and show today's statistics info

Usage: ckb-miner-stat top [OPTIONS]

Options:
  -b, --begin-block <BEGIN_BLOCK>  Chain config path [default: 15001369]
  -h, --help                       Print help
```

```
$ ckb-miner-stat top
[2025-01-06T13:12:28Z INFO  ckb_miner_stat] Current time: 2025-01-06 13:12:28.599451532 UTC
[2025-01-06T13:12:28Z INFO  ckb_miner_stat] latest block is: 15053308
[2025-01-06T13:12:28Z INFO  ckb_miner_stat] tip_block: 15053321, current hashrate 434059066082392.986654
[2025-01-06T13:12:28Z INFO  ckb_miner_stat] sync from 15053309 to 15053321
[2025-01-06T13:12:29Z INFO  ckb_miner_stat] got lines: 15
[2025-01-06T13:12:29Z INFO  ckb_miner_stat] sync done!
shape: (8, 6)
┌───────────────────────────────────────────────────────────────────────────────────────────────────┬───────┬───────────────┬─────────────┬───────────┬────────────────┐
│ Miner                                                                                             ┆ Count ┆ User_Reward   ┆ Total_Count ┆ Percent   ┆ User_Hash_Rate │
│ ---                                                                                               ┆ ---   ┆ ---           ┆ ---         ┆ ---       ┆ ---            │
│ str                                                                                               ┆ u32   ┆ f64           ┆ u32         ┆ f64       ┆ f64            │
╞═══════════════════════════════════════════════════════════════════════════════════════════════════╪═══════╪═══════════════╪═════════════╪═══════════╪════════════════╡
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqv8cuf7vzh62m9gwyv59lzha9nx6ar5d9qqkp5em ┆ 25    ┆ 15945.154668  ┆ 5104        ┆ 0.489812  ┆ 2.1261e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvzz8cmjw9pqlx48d3s9nr49fhu89jk8rgycece5 ┆ 83    ┆ 53663.956487  ┆ 5104        ┆ 1.626176  ┆ 7.0586e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqt3vn6g67jm7g5czehcmqdcr6jwjz8pdtg59p03h ┆ 89    ┆ 58199.406627  ┆ 5104        ┆ 1.74373   ┆ 7.5688e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0dn8gg6ag6uvkl0lr0xpyt0n99dsal47sm7mzyj ┆ 406   ┆ 262163.716351 ┆ 5104        ┆ 7.954545  ┆ 3.4527e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq22xdj8q4jdql98qkdhnqzgrsk4nqyavdcratvu9 ┆ 438   ┆ 282826.964071 ┆ 5104        ┆ 8.581505  ┆ 3.7249e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0259kv0x5jez6h0l5qr52k64weapapgyscsnmhn ┆ 568   ┆ 366395.651808 ┆ 5104        ┆ 11.128527 ┆ 4.8304e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvgqh40v2g5ps3rann79tvw9gq6ewrft7gp909qk ┆ 1277  ┆ 821188.661073 ┆ 5104        ┆ 25.019592 ┆ 1.0860e14      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 2218  ┆ 1.4313e6      ┆ 5104        ┆ 43.456113 ┆ 1.8863e14      │
└───────────────────────────────────────────────────────────────────────────────────────────────────┴───────┴───────────────┴─────────────┴───────────┴────────────────┘
```

### history

```
$ ckb-miner-stat history -h
show some day's statistics info

Usage: ckb-miner-stat history [OPTIONS]

Options:
  -d, --date <DATE>  the date to show history info [default: 20250101]
  -h, --help         Print help
```

```
$ ckb-miner-stat history -d 20250119
show history info for date: 2025-01-19
Blockno from 15162408 to 15171503, total 9096
Total reward: 6064740.539540419
Total Hashrate: 456782182439588.56
Avg ROR: 13.27709523858783 CKB/T
shape: (9, 5)
┌───────────────────────────────────────────────────────────────────────────────────────────────────┬───────┬───────────────┬───────────┬────────────────┐
│ Miner                                                                                             ┆ Count ┆ User_Reward   ┆ Percent   ┆ User_Hash_Rate │
│ ---                                                                                               ┆ ---   ┆ ---           ┆ ---       ┆ ---            │
│ str                                                                                               ┆ u32   ┆ f64           ┆ f64       ┆ f64            │
╞═══════════════════════════════════════════════════════════════════════════════════════════════════╪═══════╪═══════════════╪═══════════╪════════════════╡
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2rhvzx4pm3sjjpyfc9r3vz8exem0wwdksd89j70 ┆ 1     ┆ 630.665554    ┆ 0.010994  ┆ 5.0218e10      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqv8cuf7vzh62m9gwyv59lzha9nx6ar5d9qqkp5em ┆ 99    ┆ 64814.440504  ┆ 1.088391  ┆ 4.9716e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqt3vn6g67jm7g5czehcmqdcr6jwjz8pdtg59p03h ┆ 120   ┆ 78494.491987  ┆ 1.319261  ┆ 6.0262e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvzz8cmjw9pqlx48d3s9nr49fhu89jk8rgycece5 ┆ 125   ┆ 81750.326116  ┆ 1.37423   ┆ 6.2772e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0dn8gg6ag6uvkl0lr0xpyt0n99dsal47sm7mzyj ┆ 743   ┆ 497200.785081 ┆ 8.168426  ┆ 3.7312e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq22xdj8q4jdql98qkdhnqzgrsk4nqyavdcratvu9 ┆ 796   ┆ 530742.065759 ┆ 8.751099  ┆ 3.9973e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0259kv0x5jez6h0l5qr52k64weapapgyscsnmhn ┆ 1255  ┆ 836158.882704 ┆ 13.797274 ┆ 6.3023e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvgqh40v2g5ps3rann79tvw9gq6ewrft7gp909qk ┆ 2159  ┆ 1.4397e6      ┆ 23.735708 ┆ 1.0842e14      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3798  ┆ 2.5353e6      ┆ 41.754617 ┆ 1.9073e14      │
└───────────────────────────────────────────────────────────────────────────────────────────────────┴───────┴───────────────┴───────────┴────────────────┘
```

### report

```
$ ckb-miner-stat report -h
generate report about one miner

Usage: ckb-miner-stat report [OPTIONS]

Options:
  -m, --miner <MINER>  the miner to generate report [default: ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj]
  -h, --help           Print help
```

```
$ ckb-miner-stat report
[2025-01-20T05:56:09Z INFO  ckb_miner_stat] generate report for miner: ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj
[2025-01-20T05:56:09Z INFO  ckb_miner_stat] Current time: 2025-01-20 05:56:09.758502408 UTC
shape: (21, 9)
┌───────────────────────────────────────────────────────────────────────────────────────────────────┬───────┬───────────────┬───────────┬───┬─────────────┬───────────────┬───────────┬───────────────────────────────┐
│ Miner                                                                                             ┆ Count ┆ User_Reward   ┆ Percent   ┆ … ┆ Total_Count ┆ Total_Reward  ┆ Avg_ROR   ┆ Time                          │
│ ---                                                                                               ┆ ---   ┆ ---           ┆ ---       ┆   ┆ ---         ┆ ---           ┆ ---       ┆ ---                           │
│ str                                                                                               ┆ u32   ┆ f64           ┆ f64       ┆   ┆ i32         ┆ f64           ┆ f64       ┆ datetime[ns]                  │
╞═══════════════════════════════════════════════════════════════════════════════════════════════════╪═══════╪═══════════════╪═══════════╪═══╪═════════════╪═══════════════╪═══════════╪═══════════════════════════════╡
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 571   ┆ 348418.253618 ┆ 41.167988 ┆ … ┆ 1387        ┆ 845931.139676 ┆ 1.886199  ┆ 2025-01-20 05:56:09.758502408 │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3798  ┆ 2.5353e6      ┆ 41.754617 ┆ … ┆ 9096        ┆ 6.0647e6      ┆ 13.277095 ┆ 2025-01-19 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 4218  ┆ 2.5161e6      ┆ 41.284134 ┆ … ┆ 10217       ┆ 6.0995e6      ┆ 13.89048  ┆ 2025-01-18 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3502  ┆ 2.4772e6      ┆ 40.873016 ┆ … ┆ 8568        ┆ 6.0607e6      ┆ 13.819412 ┆ 2025-01-17 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3396  ┆ 2.4308e6      ┆ 39.952941 ┆ … ┆ 8500        ┆ 6.0924e6      ┆ 14.142909 ┆ 2025-01-16 23:59:59           │
│ …                                                                                                 ┆ …     ┆ …             ┆ …         ┆ … ┆ …           ┆ …             ┆ …         ┆ …                             │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 4032  ┆ 2.5470e6      ┆ 42.0      ┆ … ┆ 9600        ┆ 6.0655e6      ┆ 13.806195 ┆ 2025-01-04 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3551  ┆ 2.5454e6      ┆ 42.103391 ┆ … ┆ 8434        ┆ 6.0740e6      ┆ 13.59367  ┆ 2025-01-03 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3836  ┆ 2.5140e6      ┆ 41.461306 ┆ … ┆ 9252        ┆ 6.0676e6      ┆ 13.650736 ┆ 2025-01-02 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 3964  ┆ 2.4527e6      ┆ 40.903931 ┆ … ┆ 9691        ┆ 5.9989e6      ┆ 13.268409 ┆ 2025-01-01 23:59:59           │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 1     ┆ 562.424219    ┆ 100.0     ┆ … ┆ 1           ┆ 562.424219    ┆ 0.001271  ┆ 2024-12-31 23:59:59           │
└───────────────────────────────────────────────────────────────────────────────────────────────────┴───────┴───────────────┴───────────┴───┴─────────────┴───────────────┴───────────┴───────────────────────────────┘
```

and write report to csv file

```
$ cat data/ckb-report-ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj.csv
Miner,Count,User_Reward,Percent,User_Hash_Rate,Total_Count,Total_Reward,Avg_ROR,Time
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,571,348418.2536181697,41.16798846431146,184632055283200.4,1387,845931.13967588,1.8861991947368641,2025-01-20T05:56:09.758502408
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3798,2535293.4155539856,41.754617414248024,190727652694102.62,9096,6064740.539540419,13.27709523858783,2025-01-19T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,4218,2516109.6152211507,41.28413428599393,181283224802948.84,10217,6099464.32934165,13.890480198533764,2025-01-18T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3502,2477229.5745233484,40.87301587301587,179253293585392.78,8568,6060661.31729552,13.819411697715072,2025-01-17T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3396,2430835.160360591,39.95294117647059,172105718548924.9,8500,6092356.368596219,14.142909234676397,2025-01-16T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3946,2560225.949310417,42.180652057723144,181057354533730.62,9355,6064497.07990148,14.128365118951658,2025-01-15T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3246,2515083.147707177,41.78681771369722,180940381494051.9,7768,6013289.20930194,13.887238325354303,2025-01-14T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3471,2527807.254188424,41.38054363376252,178881787674275.47,8388,6101955.21709512,14.115591497332804,2025-01-13T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3905,2549478.5470760856,41.75577416595381,182958275069603.88,9352,6092615.375098149,13.904912012634176,2025-01-12T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3338,2450406.9809733974,40.38717483363581,174684363216638.9,8265,6064992.842196729,14.022315550864226,2025-01-11T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,2802,2399364.266699011,39.33735785483645,168212859454216.88,7123,6090544.68269749,14.243021401069011,2025-01-10T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3472,2426557.8154368964,40.46148467544575,177599578933942.75,8581,5990348.11262514,13.64746357026662,2025-01-09T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3576,2492494.0453625065,40.971585701191565,179571760762876.7,8728,6081398.25522475,13.875485139672813,2025-01-08T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3922,2452887.8559652255,40.56681836988002,178334992433163.72,9668,6048608.38245563,13.75909429180434,2025-01-07T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,4125,2584395.9733326267,42.62684716337708,187470936069861.25,9677,6056761.71592184,13.771769714393622,2025-01-06T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,4110,2533966.6552701667,41.63711883294499,185988243068906.4,9871,6094020.76086087,13.642661622239812,2025-01-05T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,4032,2546975.1778808027,42.0,184520161976695.9,9600,6065526.85390413,13.806194680023504,2025-01-04T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3551,2545394.018756647,42.103391036281714,188129150540642.72,8434,6074013.54734726,13.593670455041254,2025-01-03T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3836,2513953.269000312,41.461305663640296,184289594612255.03,9252,6067557.5543149095,13.65073589317295,2025-01-02T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3964,2452692.220187706,40.90393148281911,184934499643149.2,9691,5998901.602254841,13.268409116973308,2025-01-01T23:59:59.000000000
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,1,562.42421899,100.0,442388595130533.8,1,562.42421899,0.0012713352585955516,2024-12-31T23:59:59.000000000
```