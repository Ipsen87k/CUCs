# CUCs
cli Application

# Features

## chs(Go)
簡易的なHttp Serverをローカルで構築します。(python3 -m http.serverと似ています。)

## cplay(Rust)
mp3、wavファイルを指定し音源を再生します。

## ir(Rust)
画像ファイルやwidth、heightを指定し、指定した長さに画像をリサイズします。

## psgen(Rust)
パスワードを生成します。

## sbc(Rust)
文字列を指定し、文字数やbyte数を表示します。

## zipg(Go)
zipファイルの解凍、圧縮をします。 

# Installation
 
## Windows
ReleaseからDownloadお願いいたします。


## Linux Mac

### Rust
```bash
cargo run
```

### Go
```bash
go run .
```
 
# Usage

## chs(Go)
8080番ポートでC:/temp内のフィルを表示するhttp server作成
```bash
chs -p 8080 -d C:/temp
```

## cplay(Rust)
音源を再生
```bash
cplay hoge.wav
```

## ir(Rust)
画像を縦300、横300にリサイズ
```bash
ir hoge.jpg -w 300 -h 300
```

## psgen(Rust)
-sでパスワードの強さを指定
e>s>m>w
```bash
psgen -s e
```

-lでパスワードの長さを指定
```bash
psgen -s e -l 10
```
-cで生成するパスワードの数を指定
```bash
psgen -s e -c 2
```
-nで除外するsymbolを指定
* nu(生成するパスワードに数字を入れない)
* ns(生成するパスワードに記号を入れない)
```bash
psgen -s w -n nu
```
## sbc(Rust)
文字列のbyteを取得
```bash
sbc -b 日本語
```
文字数を取得
```bash
sbc -c Rust
```

## zipg(Go)
zipファイルを解凍
```bash
zipg -u hoge.zip -o ./go
```
zipファイルを圧縮
```bash
zipg C:\temp\a -o comp
```

# Note
### zipr
廃止予定(obsolescent)
### mp4decoder rustetris system_info_gui
作成中(in preparation)

### other
I don't test environments under Linux and Mac.


# Author
 
* Ipsen87k
 
# License
 
"CUCs" is under [MIT license](https://en.wikipedia.org/wiki/MIT_License).
