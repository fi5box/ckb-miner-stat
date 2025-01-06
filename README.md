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
look like
```
Miner,Count,Reward,Percent,Hashrate
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqv8cuf7vzh62m9gwyv59lzha9nx6ar5d9qqkp5em,49,32511.64,0.5296152,2.476971e+12
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2rhvzx4pm3sjjpyfc9r3vz8exem0wwdksd89j70,103,68343.35,1.1132728,5.206695e+12
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqt3vn6g67jm7g5czehcmqdcr6jwjz8pdtg59p03h,136,89898.11,1.4699524,6.874859e+12
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvzz8cmjw9pqlx48d3s9nr49fhu89jk8rgycece5,137,89414.99,1.4807609,6.925410e+12
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0dn8gg6ag6uvkl0lr0xpyt0n99dsal47sm7mzyj,727,482611.11,7.8577605,3.675017e+13
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq22xdj8q4jdql98qkdhnqzgrsk4nqyavdcratvu9,804,524342.79,8.6900130,4.064255e+13
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0259kv0x5jez6h0l5qr52k64weapapgyscsnmhn,1141,749987.55,12.3324687,5.767805e+13
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvgqh40v2g5ps3rann79tvw9gq6ewrft7gp909qk,2319,1516408.87,25.0648508,1.172265e+14
ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj,3836,2513781.54,41.4613057,1.939115e+14
```

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
$ ckb-miner-stat -h
ckb-miner-stat 0.1.0
david <david@fi5box.com>

Usage: ckb-miner-stat <COMMAND>

Commands:
  top      sync data from chain and show today's statistics info
  history  show some day's statistics info
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
$ ckb-miner-stat history -d 20250104
show history info for date: 2025-01-04
shape: (8, 6)
┌───────────────────────────────────────────────────────────────────────────────────────────────────┬───────┬───────────────┬─────────────┬───────────┬────────────────┐
│ Miner                                                                                             ┆ Count ┆ User_Reward   ┆ Total_Count ┆ Percent   ┆ User_Hash_Rate │
│ ---                                                                                               ┆ ---   ┆ ---           ┆ ---         ┆ ---       ┆ ---            │
│ str                                                                                               ┆ u32   ┆ f64           ┆ u32         ┆ f64       ┆ f64            │
╞═══════════════════════════════════════════════════════════════════════════════════════════════════╪═══════╪═══════════════╪═════════════╪═══════════╪════════════════╡
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqv8cuf7vzh62m9gwyv59lzha9nx6ar5d9qqkp5em ┆ 41    ┆ 25961.235496  ┆ 9600        ┆ 0.427083  ┆ 1.9077e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqt3vn6g67jm7g5czehcmqdcr6jwjz8pdtg59p03h ┆ 125   ┆ 78593.945498  ┆ 9600        ┆ 1.302083  ┆ 5.8163e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvzz8cmjw9pqlx48d3s9nr49fhu89jk8rgycece5 ┆ 149   ┆ 93822.130516  ┆ 9600        ┆ 1.552083  ┆ 6.9330e12      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0dn8gg6ag6uvkl0lr0xpyt0n99dsal47sm7mzyj ┆ 801   ┆ 505831.745397 ┆ 9600        ┆ 8.34375   ┆ 3.7271e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq22xdj8q4jdql98qkdhnqzgrsk4nqyavdcratvu9 ┆ 813   ┆ 514374.496015 ┆ 9600        ┆ 8.46875   ┆ 3.7829e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0259kv0x5jez6h0l5qr52k64weapapgyscsnmhn ┆ 1249  ┆ 790547.790185 ┆ 9600        ┆ 13.010417 ┆ 5.8116e13      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqvgqh40v2g5ps3rann79tvw9gq6ewrft7gp909qk ┆ 2390  ┆ 1.5094e6      ┆ 9600        ┆ 24.895833 ┆ 1.1121e14      │
│ ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq0tpsqq08mkay9ewrfrdwlcghv62qw704s93hhsj ┆ 4032  ┆ 2.5470e6      ┆ 9600        ┆ 42.0      ┆ 1.8761e14      │
└───────────────────────────────────────────────────────────────────────────────────────────────────┴───────┴───────────────┴─────────────┴───────────┴────────────────┘
```