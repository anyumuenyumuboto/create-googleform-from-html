
# 2024/11/23
## reference 

[curlコマンドで必要なものだけ表示する | クロジカ](https://tech.kurojica.com/archives/51791/)

[Bashのスクリプトをちょっとした手間でdry-runに対応したい #Bash - Qiita](https://qiita.com/fukasawah/items/2bd934c726442624c16f)
--dry-run

[tmuxでconsoleのスクロール(not mouse)を行う方法 #tmux - Qiita](https://qiita.com/sutoh/items/41ddd9bdbc9e23746c9d)

# 2025年1月5日

## todo

- [ x ] googleform サブコマンドを作る
ref: [clapの使い方まとめ - ぽよメモ](https://poyo.hatenablog.jp/entry/2022/10/10/170000#f-75a0c979)

- [ ] env_loggerを使う。エラーメッセージのファイル名と行数も出力するようにする。

# 2025年1月11日

ref [Rustのログ出力，logクレートとenv_loggerクレートについて （Actix_webのログ出力も含む）](https://zenn.dev/neruneruna7/articles/52e7753bc66b0b)

>`RUST_LOG=debug cargo run`とすれば，debugレベルに設定して実行することができます．

ref [Rust：logでログ出力を行う #Rust - Qiita](https://qiita.com/fujitayy/items/590145c0f4b4e7d06de7)

>env_loggerの標準のフォーマットでは不足がある場合はカスタマイズする事ができます。
ここではログ出力を行ったファイル名と行番号を足してみましょう。
```rust
use env_logger;
use log::{error, warn, info, debug};
use std::env;
use std::io::Write;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {}] {} {}:{}",
                ts,
                record.level(),
                record.target(),
                record.args(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
            )
        })
        .init();
    
    debug!("debugです");
    info!("infoです");
    warn!("warnです");
    error!("errorです");
}
```

# 20240114
```bash
cargo run -q -- -i README.md googleform -i $CLIENT_ID -s $CLIENT_SECRET -f $FORM_ID
```
