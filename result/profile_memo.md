## Profiling memo

Rust と C で有意に差が出ているもの

### matmult_int
SIMD 化の有無がそのまま速度の差として出ている。

Backend 同じだけどSIMD化のされやすさってこんなに変わるもんなんかな？

C version のアセンブリ
- 64bit SIMD

Rust version のアセンブリ
- SIMDなし
- ループロールのみ

### aha-mont64
単純に計測すると、Rust版の方が命令数が圧倒的に少ない

- C version: 6,283,072,807
- Rust version: 3,345,787,339

- 比率にして 1.87

つまり、C言語版で出ている命令数が単純に多い

なぜ命令数が少ないか？

Rustの逆アセンブル結果を見ると、謎の即値が大量に見える

inline を抑制するとRust版の方が遅い
- inline されると抑制されるタイプの最適化

ぱっと見コンパイル時計算の類の最適化が働いている

``` rust
mulul64(a, b, &mut p1hi, &mut p1lo);
p1 = modul64(p1hi, p1lo, m);
mulul64(p1, p1, &mut p1hi, &mut p1lo); // <-- ここまでの計算をコンパイル時に実行し、即値に置き換えるっぽい
p1 = modul64(p1hi, p1lo, m);
mulul64(p1, p1, &mut p1hi, &mut p1lo);
p1 = modul64(p1hi, p1lo, m);
```

この即値置き換えを行うことでかなりの命令を削減している

### crc32
link time 最適化が不十分であることによる。
Cの方でも、embenchのデフォルト以外でlink time最適化を有効にすることでRustに匹敵する性能が出ることがわかった
- -flto をつければね
逆にRustの場合crateを跨がない限りはLTOが効くみたい。
- crateを跨がない限りはLink time 最適化がきく

### cubic
- 大雑把に3つの要因が別れる
    - sqrt の仕様の問題 (C言語の場合にはsqrtの引数がマイナスの場合に errno として 33 (EDOM: Math argument out of domain of func) が設定される)
        - -ffast-math で一応回避され、余計なsqrt関数callの分岐は全て消える
    - vsqrt.f64 命令を呼ぶにしても、errnoを設定しないといけない場合には、sqrt へ分岐する必要性が出てくる
    - atan がなぜかcallされている？ <- 元の実装が原因。PIをatanから定義していたため (これはプルリクをだす)
    - inline されていないこと (ltoがないことによるもの)

### matmult-int
- 要因は一つ
- SIMDかしてくれなかったところ
- サイズがもう少し大きければ、もっと差が出てくるはず

### nbody
- 副作用がない関数呼び出しは不要であることをコンパイラが見抜いたことにより、呼び出し自体を完全に削除してしまった
- バックエンドが同じであれば、同様の最適化を行ってくれる

