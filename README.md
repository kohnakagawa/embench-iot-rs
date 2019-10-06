

## memo
- trait を使ったうまくベンチマーク部分を抽象的に実装してみる
- 全移植は結構時間がかかりそうなので、発表用のネタ作りは少し対象を絞る
- 以下を移植の対象として絞ることにする

| name        | branch | memory | compute | status |
|-------------|--------|--------|---------|--------|
| aha-mont64  | low    | low    | high    |        |
| crc32       | high   | med    | low     |        |
| st          | med    | low    | high    |        |
| nettle-aes  | med    | high   | low     |        |
| mutmult-int | med    | med    | med     |        |
| cubic       | low    | med    | med     |        |
| minver      | high   | low    | med     |        |

- edn と huffbench は後ほど移植
- 上記に絞り、ベンチマーク移植を行う

