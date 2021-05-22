# reverse polish
逆ポーランド電卓です。Rustの学習用に作りました。　

## How to Use

### Std Input
```shell
cd path/to/reverse_polish

cargo run -- 
1 1 *
```

### File Input
計算式を入力したファイルを作成する。
```shell
echo "1 1 *" > input.txt
```

コマンド実行の際に、ファイルを指定する。
```shell
cd path/to/reverse_polish

cargo run -- path/to/input.txt
```
