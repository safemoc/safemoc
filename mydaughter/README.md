# mydaughter

Rust 调用 DeepSeek 的最小命令行示例，使用 OpenAI 兼容的 Chat Completions 格式。

## 运行

在项目根目录 `safemoc` 的 PowerShell 中执行：

```powershell
$env:DEEPSEEK_API_KEY = "你的 DeepSeek API Key"
cargo run -p mydaughter -- "Rust 的所有权是什么？"
```

密钥在 [DeepSeek 平台](https://platform.deepseek.com/)申请，通过环境变量提供。程序不自动读取 `.env` 文件。

默认模型为 `deepseek-flash`，可以通过环境变量覆盖：

```powershell
$env:DEEPSEEK_MODEL = "deepseek-v4-pro"
cargo run -p mydaughter -- "你好"
```

查看帮助（无需密钥，不发起 API 调用）：

```powershell
cargo run -p mydaughter -- --help
```

## 调用过程

1. `reqwest` 向 `https://api.deepseek.com/chat/completions` 发送 POST 请求。
2. `Authorization: Bearer ...` 携带 DeepSeek 密钥。
3. JSON 请求体提供 `model`、`messages` 和 `stream: false`。
4. 从响应的 `choices[0].message.content` 读取回答。

`serde_json` 构造请求 JSON，`serde` 将响应转换成 Rust 结构体。
本示例使用阻塞请求、关闭思考模式，输出上限为 1024 tokens；每次运行是一次独立问答。
要实现多轮对话，需要保存并在下次请求的 `messages` 中带上历史消息。
服务端或并发场景可改用异步 `reqwest::Client` 和 Tokio。

官方文档：[首次调用 API](https://api-docs.deepseek.com/zh-cn/)、[Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)。
