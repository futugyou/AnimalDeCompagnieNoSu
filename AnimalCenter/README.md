https://actix.rs/docs/getting-started/

##### git
```
{user}/.cargo
create config

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

##### if git not support
```
{user}/.cargo
create config

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "http://mirrors.ustc.edu.cn/crates.io-index"
```

cargo vendor