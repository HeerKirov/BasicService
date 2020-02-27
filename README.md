# BasicService
将用户相关的服务抽离出来，为其他需要用户系统和用户认证的应用程序提供登录服务。

## Technology
* Rust (>=1.41.0)
* actix-web
* r2d2
* postgres

## How to use

### Setup
#### 1. 编译程序
```bash
cargo build --release
```
#### 2. 依赖工具
`convert`: 程序的运行依赖于ImageMagick的`convert`CLI工具。在运行程序之前，需要准备好该工具，并使其处于运行用户可访问的状态。

### Initialize
1. 创建配置文件`config.properties`并编写配置。  
   文件模版见`config.properties.example`。

2. 创建postgres数据库，并使用`migration`文件夹下的SQL文件初始化数据库结构。

3. 运行
    ```bash
    cargo run initialize-datasource
    ```
    将必备的基本数据写入到数据库。

### Run
```bash
cargo run runserver
```

### Log Configuration
项目使用`log`库输出日志。

通过`export RUST_LOG={level}`，或者`RUST_LOG={level} cargo run`，来定义需要的log级别。

* error：只打印运行中的错误。不设置env时，默认位于此级别下。
* warn：打印有问题的警告性信息。
* info: 打印大多数运行信息。建议的级别。