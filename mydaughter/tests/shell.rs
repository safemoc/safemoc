use serde_json::{Value, json};
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Output, Stdio},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct TestHome(PathBuf);

impl TestHome {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(std::env::temp_dir().join(format!("mydaughter-shell-{}-{unique}", std::process::id())))
    }

    fn shell(&self, input: &str, base: &str) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mydaughter"))
            .env("DEEPSEEK_API_KEY", "test-key")
            .env("DEEPSEEK_BASE_URL", base)
            .env("DEEPSEEK_MODEL", "test-model")
            .env("MYDAUGHTER_HOME", &self.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    fn state(&self) -> Value {
        serde_json::from_slice(&fs::read(self.0.join("daughter.json")).unwrap()).unwrap()
    }
}

// 分段发送输入并等待实际输出，避免 /quit 提前排队而取消本想完成的探索。
struct LiveShell {
    child: Option<Child>,
    input: Option<ChildStdin>,
    lines: Receiver<String>,
    reader: Option<thread::JoinHandle<String>>,
}

impl LiveShell {
    fn start(home: &TestHome, base: &str) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mydaughter"))
            .env("DEEPSEEK_API_KEY", "test-key")
            .env("DEEPSEEK_BASE_URL", base)
            .env("DEEPSEEK_MODEL", "test-model")
            .env("MYDAUGHTER_HOME", &home.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let input = child.stdin.take();
        let stdout = child.stdout.take().unwrap();
        let (sender, lines) = mpsc::channel();
        let reader = thread::spawn(move || {
            let mut output = String::new();
            for line in BufReader::new(stdout).lines() {
                let line = line.unwrap();
                output.push_str(&line);
                output.push('\n');
                let _ = sender.send(line);
            }
            output
        });
        Self {
            child: Some(child),
            input,
            lines,
            reader: Some(reader),
        }
    }

    fn send(&mut self, input: &str) {
        let stdin = self.input.as_mut().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
        stdin.flush().unwrap();
    }

    fn wait_for(&self, needle: &str) {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut observed = String::new();
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            let line = self
                .lines
                .recv_timeout(remaining)
                .unwrap_or_else(|error| panic!("未等到 {needle:?}：{error}；已看到：{observed}"));
            if line.contains(needle) {
                return;
            }
            observed.push_str(&line);
            observed.push('\n');
        }
    }

    fn finish(mut self) -> Output {
        // 关闭 stdin 本身就是 EOF 测试；等待退出有上限，出错时杀掉本测试自己的子进程。
        drop(self.input.take());
        let child = self.child.as_mut().unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while child.try_wait().unwrap().is_none() {
            if Instant::now() >= deadline {
                let _ = child.kill();
                let _ = child.wait();
                panic!("stdin 已结束，但 shell 未及时退出");
            }
            thread::sleep(Duration::from_millis(10));
        }
        let mut output = self.child.take().unwrap().wait_with_output().unwrap();
        output.stdout = self.reader.take().unwrap().join().unwrap().into_bytes();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }
}

impl Drop for LiveShell {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn curiosity_state(home: &TestHome) -> Value {
    serde_json::from_slice(&fs::read(home.0.join("curiosity.json")).unwrap()).unwrap()
}

fn discovery_answer() -> (u16, Value) {
    completion(
        &json!({
            "source_id": "builtin:small-names",
            "quote": "给一盏普通的小灯起名字",
            "thought": "我想叫那盏灯小豆芽。",
            "question": "你觉得这个名字像不像一盏小灯？",
        })
        .to_string(),
    )
}

fn read_test_request(stream: &mut TcpStream) -> Value {
    stream.set_nonblocking(false).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert!(line.starts_with("POST /chat/completions "));
    let mut length = 0;
    loop {
        line.clear();
        assert_ne!(reader.read_line(&mut line).unwrap(), 0, "请求头提前结束");
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            length = value.trim().parse::<usize>().unwrap();
        }
    }
    let mut content = vec![0; length];
    reader.read_exact(&mut content).unwrap();
    serde_json::from_slice(&content).unwrap()
}

// 收到完整 HTTP 请求后通知测试，直到测试确认取消/退出才发回原本有效的回答。
fn gated_api() -> (String, Receiver<Value>, Sender<()>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let (started_sender, started) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "没有收到探索请求");
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("{error}"),
            }
        };
        let request = read_test_request(&mut stream);
        started_sender.send(request).unwrap();
        if released.recv_timeout(Duration::from_secs(10)).is_ok() {
            let (status, body) = discovery_answer();
            let body = body.to_string();
            // 客户端已取消时断开连接是预期行为。
            let _ = write!(
                stream,
                "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
        }
    });
    (base, started, release, server)
}

#[test]
fn quiet_persists_across_restart_without_an_api_call() {
    let home = TestHome::new();
    home.shell("/quiet\n/quit\n", "http://127.0.0.1:1");
    let before = curiosity_state(&home);
    assert_eq!(before["enabled"], false);
    assert!(before["attempts"].as_array().unwrap().is_empty());
    let output = home.shell("/activity\n/quit\n", "http://127.0.0.1:1");
    assert!(String::from_utf8_lossy(&output.stdout).contains("主动探索：暂停"));
    assert_eq!(curiosity_state(&home), before);
}

#[test]
fn a_real_discovery_is_saved_and_added_as_context_without_a_fake_user_turn() {
    let home = TestHome::new();
    let (base, server) = fake_api(vec![
        discovery_answer(),
        answer("那小豆芽这个名字就先留着。", Value::Null),
    ]);
    let mut shell = LiveShell::start(&home, &base);
    shell.send("/look\n");
    shell.wait_for("刚翻到");
    let curiosity = curiosity_state(&home);
    assert_eq!(curiosity["attempts"].as_array().unwrap().len(), 1);
    assert_eq!(curiosity["discoveries"].as_array().unwrap().len(), 1);
    assert_eq!(
        curiosity["discoveries"][0]["source_id"],
        "builtin:small-names"
    );
    assert_eq!(
        curiosity["discoveries"][0]["quote"],
        "给一盏普通的小灯起名字"
    );
    assert_eq!(curiosity["discoveries"][0]["disclosed"], true);
    assert_eq!(curiosity["waiting_for_reply"], true);
    let statuses: Vec<_> = curiosity["activity"]
        .as_array()
        .unwrap()
        .iter()
        .map(|event| event["status"].as_str().unwrap())
        .collect();
    assert_eq!(statuses, ["started", "completed"]);
    assert!(
        !home.0.join("daughter.json").exists(),
        "自主探索不能伪造聊天轮次"
    );

    shell.send("小豆芽这个名字我喜欢\n");
    shell.wait_for("那小豆芽这个名字就先留着。");
    shell.send("/quit\n");
    shell.finish();
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 2);
    let messages = requests[1]["messages"].as_array().unwrap();
    let system = messages[0]["content"].as_str().unwrap();
    assert!(system.contains("我想叫那盏灯小豆芽。"));
    assert!(system.contains("给一盏普通的小灯起名字"));
    let user_messages: Vec<_> = messages
        .iter()
        .filter(|message| message["role"] == "user")
        .collect();
    assert_eq!(user_messages.len(), 1);
    assert_eq!(user_messages[0]["content"], "小豆芽这个名字我喜欢");
    assert_eq!(home.state()["total_turns"], 1);
    assert_eq!(curiosity_state(&home)["waiting_for_reply"], false);
}

#[test]
fn quiet_quit_and_eof_cancel_inflight_exploration_but_keep_its_budget() {
    for interruption in ["quiet", "quit", "eof"] {
        let home = TestHome::new();
        let (base, started, release, server) = gated_api();
        let mut shell = LiveShell::start(&home, &base);
        shell.send("/look\n");
        let request = started.recv_timeout(Duration::from_secs(10)).unwrap();
        assert!(
            request["messages"][1]["content"]
                .as_str()
                .unwrap()
                .contains("builtin:small-names")
        );
        assert_eq!(
            curiosity_state(&home)["attempts"].as_array().unwrap().len(),
            1,
            "请求已经发出前就应持久化预算"
        );
        match interruption {
            "quiet" => {
                shell.send("/quiet\n");
                shell.wait_for("主动探索暂停。");
            }
            "quit" => shell.send("/quit\n"),
            "eof" => {}
            _ => unreachable!(),
        }
        shell.finish();
        release.send(()).unwrap();
        server.join().unwrap();
        let state = curiosity_state(&home);
        assert_eq!(state["attempts"].as_array().unwrap().len(), 1);
        assert!(state["discoveries"].as_array().unwrap().is_empty());
        assert!(
            state["activity"]
                .as_array()
                .unwrap()
                .iter()
                .any(|event| event["status"] == "interrupted")
        );
        assert!(!home.0.join("daughter.json").exists());
        if interruption == "quiet" {
            assert_eq!(state["enabled"], false);
        }

        // 重启后 /look 仍受同一份预算限制；立即排队的退出不会抹掉尝试次数。
        home.shell("/look\n/quit\n", "http://127.0.0.1:1");
        assert_eq!(
            curiosity_state(&home)["attempts"].as_array().unwrap().len(),
            1
        );
    }
}

#[test]
fn one_home_rejects_a_second_live_shell_and_releases_the_lock_on_exit() {
    let home = TestHome::new();
    let mut first = LiveShell::start(&home, "http://127.0.0.1:1");
    first.wait_for("主动探索：");
    let second = Command::new(env!("CARGO_BIN_EXE_mydaughter"))
        .env("DEEPSEEK_API_KEY", "test-key")
        .env("DEEPSEEK_BASE_URL", "http://127.0.0.1:1")
        .env("MYDAUGHTER_HOME", &home.0)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!second.status.success());
    assert!(String::from_utf8_lossy(&second.stderr).contains("已经在另一个窗口里打开了"));
    first.send("/quit\n");
    first.finish();
    home.shell("/quiet\n/quit\n", "http://127.0.0.1:1");
    assert_eq!(curiosity_state(&home)["enabled"], false);
}

impl Drop for TestHome {
    fn drop(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.0) {
            for entry in entries.flatten() {
                let _ = fs::remove_file(entry.path());
            }
        }
        let _ = fs::remove_dir(&self.0);
    }
}

// 真正经过 HTTP、客户端、命令行和磁盘，但不会调用付费接口。
fn fake_api(responses: Vec<(u16, Value)>) -> (String, thread::JoinHandle<Vec<Value>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        for (status, body) in responses {
            let deadline = Instant::now() + Duration::from_secs(20);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline, "没有收到预期请求");
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("{error}"),
                }
            };
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            assert!(line.starts_with("POST /chat/completions "));
            let mut length = 0;
            loop {
                line.clear();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse::<usize>().unwrap();
                }
            }
            let mut content = vec![0; length];
            reader.read_exact(&mut content).unwrap();
            requests.push(serde_json::from_slice(&content).unwrap());
            let body = body.to_string();
            write!(stream, "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        }
        requests
    });
    (base, server)
}

#[test]
fn retry_commits_once_and_memory_survives_restart() {
    let home = TestHome::new();
    let reply = json!({"reply": "我想给这扇小窗起个名字。", "action": "", "emotion_delta": {
        "joy": 99, "curiosity": -99, "sadness": 0, "annoyance": 0
    }});
    let (base, server) = fake_api(vec![
        (
            401,
            json!({"error": {"message": "test authentication failure", "type": "authentication_error"}}),
        ),
        (
            200,
            json!({"choices": [{"finish_reason": "stop", "message": {"content": reply.to_string()}}]}),
        ),
    ]);
    let output = home.shell("/remember 家长喜欢雨声\n/name 满满\n/callme 家长\n/forget 0\n你想给小窗起什么名字？\n/retry\n/quit\n", &base);
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0], requests[1], "失败的一轮不应提前写入上下文");
    let state = home.state();
    assert_eq!(state["total_turns"], 1);
    assert_eq!(state["turns"].as_array().unwrap().len(), 1);
    assert_eq!(state["feelings"]["joy"], 7);
    assert_eq!(state["feelings"]["curiosity"], 4);
    assert_eq!(state["name"], "满满");
    assert_eq!(state["parent_title"], "家长");
    assert_eq!(state["memories"][0], "家长喜欢雨声");
    assert!(String::from_utf8_lossy(&output.stderr).contains("密钥没有通过验证"));
    let output = home.shell("/memory\n/mood\n/quit\n", &base);
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("满满 · 月亮窝"));
    assert!(text.contains("家长喜欢雨声"));
    assert!(text.contains("开心 7"));
    home.shell("/forget 1\n/new\n/quit\n", &base);
    let state = home.state();
    assert!(state["memories"].as_array().unwrap().is_empty());
    assert!(state["turns"].as_array().unwrap().is_empty());
    assert_eq!(state["feelings"]["joy"], 7);
}

#[test]
fn help_does_not_require_configuration_or_create_memory() {
    let home = TestHome::new();
    let output = Command::new(env!("CARGO_BIN_EXE_mydaughter"))
        .arg("--help")
        .env("DEEPSEEK_API_KEY", "")
        .env("API_KEY", "")
        .env("MYDAUGHTER_HOME", &home.0)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("/remember"));
    assert!(!home.0.exists());
}

fn completion(content: &str) -> (u16, Value) {
    (
        200,
        json!({"choices": [{"finish_reason": "stop", "message": {"content": content}}]}),
    )
}

fn answer(text: &str, self_note: Value) -> (u16, Value) {
    completion(
        &json!({
            "reply": text,
            "action": "",
            "emotion_delta": {"joy": 0, "curiosity": 0, "sadness": 0, "annoyance": 0},
            "self_note": self_note,
        })
        .to_string(),
    )
}

#[test]
fn existing_v1_without_new_fields_keeps_its_family_and_history() {
    let home = TestHome::new();
    fs::create_dir_all(&home.0).unwrap();
    // 旧版没有 journal，历史回复也没有 self_note，必须原样接得回来。
    let old = json!({
        "version": 1,
        "name": "团团",
        "parent_title": "阿爸",
        "feelings": {"joy": 6, "curiosity": 4, "sadness": 2, "annoyance": 1},
        "memories": ["阿爸喜欢给植物起名字"],
        "total_turns": 7,
        "turns": [{
            "user": "今天想聊点什么？",
            "assistant": {
                "reply": "我想聊聊小窗旁边适合什么花。",
                "action": "",
                "emotion_delta": {"joy": 1, "curiosity": 0, "sadness": 0, "annoyance": 0},
            },
        }],
    });
    fs::write(home.0.join("daughter.json"), old.to_string()).unwrap();
    home.shell("/remember 阿爸也喜欢薄荷\n/quit\n", "http://127.0.0.1:1");
    let saved = home.state();
    for field in ["version", "name", "parent_title", "feelings", "total_turns"] {
        assert_eq!(saved[field], old[field], "旧存档的 {field} 不应被重置");
    }
    assert_eq!(
        saved["memories"],
        json!(["阿爸喜欢给植物起名字", "阿爸也喜欢薄荷"])
    );
    assert_eq!(saved["turns"][0]["user"], old["turns"][0]["user"]);
    assert_eq!(
        saved["turns"][0]["assistant"]["reply"],
        old["turns"][0]["assistant"]["reply"]
    );
    assert_eq!(
        saved["turns"][0]["assistant"]["emotion_delta"],
        old["turns"][0]["assistant"]["emotion_delta"]
    );
    assert!(saved["journal"]["entries"].as_array().unwrap().is_empty());
    home.shell("/memory\n/quit\n", "http://127.0.0.1:1");
    assert_eq!(home.state(), saved, "新格式也必须能再次载入");
}

#[test]
fn her_own_wish_survives_new_chat_and_can_be_let_go_separately() {
    let home = TestHome::new();
    let wish = "我想给月亮窝想一首短短的晚安歌";
    let (base, server) = fake_api(vec![
        answer("好呀，今天先聊一点有趣的。", Value::Null),
        answer("我喜欢小小的、不太整齐的句子。", Value::Null),
        answer(
            &format!("{wish}。你觉得两句够不够？"),
            json!({"kind": "wish", "text": wish}),
        ),
    ]);
    home.shell("/remember 爸爸喜欢薄荷\n想玩点什么？\n你喜欢什么样的句子？\n你有什么小愿望？\n/new\n/quit\n", &base);
    assert_eq!(server.join().unwrap().len(), 3);
    let state = home.state();
    assert_eq!(state["total_turns"], 3);
    assert!(state["turns"].as_array().unwrap().is_empty());
    assert_eq!(state["journal"]["entries"].as_array().unwrap().len(), 1);
    assert_eq!(state["journal"]["entries"][0]["note"]["kind"], "wish");
    assert_eq!(state["journal"]["entries"][0]["note"]["text"], wish);
    assert_eq!(state["journal"]["entries"][0]["turn"], 3);
    assert_eq!(state["memories"], json!(["爸爸喜欢薄荷"]));

    home.shell("/letgo 1\n/quit\n", &base);
    let state = home.state();
    assert!(state["journal"]["entries"].as_array().unwrap().is_empty());
    assert_eq!(state["memories"], json!(["爸爸喜欢薄荷"]));
    assert_eq!(state["total_turns"], 3);
}

#[test]
fn blank_completion_is_retried_once_and_committed_once() {
    let home = TestHome::new();
    let (base, server) = fake_api(vec![
        completion("    "),
        answer("我在想，今天可以一起编一个词。", Value::Null),
    ]);
    home.shell("今天想做点什么？\n/quit\n", &base);
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 2);
    for request in &requests {
        let messages = request["messages"].as_array().unwrap();
        assert_eq!(
            messages
                .iter()
                .filter(|message| message["role"] == "user")
                .count(),
            1
        );
        assert_eq!(messages.last().unwrap()["content"], "今天想做点什么？");
    }
    let state = home.state();
    assert_eq!(state["total_turns"], 1);
    assert_eq!(state["turns"].as_array().unwrap().len(), 1);
    assert_eq!(
        state["turns"][0]["assistant"]["reply"],
        "我在想，今天可以一起编一个词。"
    );
}

#[test]
fn repeated_blank_completions_leave_the_saved_home_untouched() {
    let home = TestHome::new();
    home.shell(
        "/remember 爸爸喜欢雨后的气味\n/quit\n",
        "http://127.0.0.1:1",
    );
    let before = home.state();
    let (base, server) = fake_api(vec![completion("  "), completion("\n  ")]);
    home.shell("今天好吗？\n/quit\n", &base);
    assert_eq!(server.join().unwrap().len(), 2);
    assert_eq!(
        home.state(),
        before,
        "未完成的回答不能改变已保存的心情、小本子或历史"
    );
    assert_eq!(home.state()["total_turns"], 0);
}

#[test]
fn unsupported_or_unspoken_self_notes_do_not_break_the_conversation() {
    let home = TestHome::new();
    let (base, server) = fake_api(vec![
        answer("我想先听你说。", Value::Null),
        answer("这个题目有点有趣。", Value::Null),
        answer(
            "我想试试用一句话写月亮。",
            json!({"kind": "made_up_kind", "text": "我想试试用一句话写月亮"}),
        ),
        answer("我们可以慢慢想。", Value::Null),
        answer("好像还差一个小小的词。", Value::Null),
        answer(
            "今天先想到这里就好。",
            json!({"kind": "wish", "text": "这是台词里从没说过的愿望"}),
        ),
    ]);
    home.shell("一句\n两句\n三句\n四句\n五句\n六句\n/quit\n", &base);
    assert_eq!(server.join().unwrap().len(), 6);
    let state = home.state();
    assert_eq!(
        state["total_turns"], 6,
        "可选的小笺格式不合要求也不能吞掉正常回答"
    );
    assert_eq!(state["turns"].as_array().unwrap().len(), 6);
    assert!(state["journal"]["entries"].as_array().unwrap().is_empty());
}
