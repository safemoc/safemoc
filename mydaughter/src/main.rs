mod bookshelf;
mod chat;
mod curiosity;
mod daughter;
mod growth;
mod storage;

use chat::Chat;
use daughter::{Daughter, MAX_INPUT, terminal_text, valid_text};
use std::{
    env,
    io::{self, IsTerminal, Write},
    process::ExitCode,
};
use storage::Home;

const HELP: &str = "mydaughter · 小满的月亮窝

运行：cargo run -p mydaughter
说一句：cargo run -p mydaughter -- \"今天过得怎么样？\"

直接打字，按回车聊天。名字和称呼都可以慢慢改。
  /help              看看这些小纸条
  /mood              看看她现在的心情
  /memory            翻翻长期记忆的小本子
  /journal           看她留下的小偏爱、小愿望和领悟
  /letgo 编号        放下一条成长小笺
  /letter            读读妈妈写给她的信
  /bookshelf         看看她可以阅读的资料架
  /share 文件路径    分享一份 md/txt（探索时会发送给模型服务）
  /look              让她现在自己翻一翻资料架
  /curious           开启空闲时主动探索
  /quiet             暂停主动探索
  /activity          查看她实际做过的活动
  /discoveries       查看她读到了什么、留下了什么
  /remember 内容     把一件事认真记下来（最多 200 字）
  /forget 编号       忘掉小本子中的一条；最近聊天中仍可能提到
  /callme 称呼       以后怎样称呼你，例如 /callme 妈妈
  /name 名字         给她换个小名
  /new               放下最近的聊天，保留心情与长期记忆
  /retry             重试刚才没能回答的那句话
  /quit              说声再见（也可以 /exit）

配置：DEEPSEEK_API_KEY（兼容 API_KEY）、DEEPSEEK_MODEL、DEEPSEEK_BASE_URL。
自动读取 .env。默认模型 deepseek-flash。
记忆默认保存在 mydaughter/.mydaughter/daughter.json，可用 MYDAUGHTER_HOME 更改目录。
每次成功聊天或修改小本子都会保存，关掉终端也不会丢失已保存的内容。";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{HELP}");
        return Ok(());
    }
    chat::load_env()?;
    let home = Home::new(chat::data_dir());
    let _session_lock = home.lock()?;
    let mut daughter = home.load()?;
    let mut curiosity = home.load_curiosity()?;
    let client = Chat::from_env();
    if !args.is_empty() {
        let input = args.join(" ");
        check_input(&input)?;
        acknowledge(&home, &mut curiosity)?;
        speak(&client, &home, &mut daughter, &input, &curiosity).await?;
        return Ok(());
    }

    let interactive = io::stdin().is_terminal();
    println!(
        "\n  {} · 月亮窝\n  输入 /help 看小纸条，/quit 退出。",
        daughter.name
    );
    println!(
        "  主动探索：{}。只在本程序运行时翻自己的资料架，/quiet 可以暂停。\n",
        if curiosity.enabled {
            "开启"
        } else {
            "暂停"
        }
    );
    let mut inbox = input_channel();
    let mut next_explore =
        tokio::time::Instant::now() + std::time::Duration::from_secs(curiosity::IDLE_SECONDS);
    let mut pending: Option<String> = None;
    let mut received = None;
    loop {
        if interactive {
            print!("你 > ");
            io::stdout()
                .flush()
                .map_err(|e| format!("终端输出失败：{e}"))?;
        }
        let incoming = if let Some(line) = received.take() {
            line
        } else {
            tokio::select! {
                biased;
                line = inbox.recv() => line,
                _ = tokio::time::sleep_until(next_explore), if interactive && curiosity.enabled && client.configured() => {
                    next_explore = tokio::time::Instant::now() + std::time::Duration::from_secs(curiosity::COOLDOWN_SECONDS);
                    match explore(&client, &home, &daughter, &mut curiosity, &mut inbox, false).await {
                        Ok(Explored::Input(line)) => line,
                        Ok(Explored::Finished) => continue,
                        Err(error) => { eprintln!("（{error}）"); curiosity.enabled = false; continue; }
                    }
                }
            }
        };
        let Some(line) = incoming else {
            break;
        };
        let line = line.map_err(|e| format!("没读到输入：{e}"))?;
        next_explore =
            tokio::time::Instant::now() + std::time::Duration::from_secs(curiosity::IDLE_SECONDS);
        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if matches!(input, "/quit" | "/exit") {
            break;
        }
        if input == "/look" {
            match explore(&client, &home, &daughter, &mut curiosity, &mut inbox, true).await {
                Ok(Explored::Input(line)) => received = Some(line),
                Ok(Explored::Finished) => {}
                Err(error) => eprintln!("（{error}）\n"),
            }
            continue;
        }
        if input == "/retry" {
            let Some(text) = pending.clone() else {
                println!("（没有等待重试的话，直接说就好。）\n");
                continue;
            };
            match speak(&client, &home, &mut daughter, &text, &curiosity).await {
                Ok(()) => pending = None,
                Err(error) => eprintln!("（{error}）\n"),
            }
            continue;
        }
        if input.starts_with('/') {
            match curiosity_command(input, &home, &mut curiosity) {
                Ok(true) => continue,
                Ok(false) => {}
                Err(error) => {
                    eprintln!("（{error}）\n");
                    continue;
                }
            }
            match command(input, &home, &mut daughter) {
                Ok(clear_pending) => {
                    if clear_pending {
                        pending = None;
                    }
                }
                Err(error) => eprintln!("（{error}）\n"),
            }
            continue;
        }
        if let Err(error) = check_input(input) {
            eprintln!("（{error}）\n");
            continue;
        }
        acknowledge(&home, &mut curiosity)?;
        pending = Some(input.into());
        match speak(&client, &home, &mut daughter, input, &curiosity).await {
            Ok(()) => pending = None,
            Err(error) => eprintln!("（{error}）\n"),
        }
    }
    Ok(())
}

type Input = Result<String, io::Error>;
fn input_channel() -> tokio::sync::mpsc::Receiver<Input> {
    let (sender, receiver) = tokio::sync::mpsc::channel(16);
    // 独立标准线程阻塞读取；不使用会阻止 Tokio runtime 退出的 blocking task。
    std::thread::spawn(move || {
        use std::io::BufRead;
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let failed = line.is_err();
            if sender.blocking_send(line).is_err() || failed {
                break;
            }
        }
    });
    receiver
}

enum Explored {
    Finished,
    Input(Option<Input>),
}

async fn explore(
    client: &Chat,
    home: &Home,
    daughter: &Daughter,
    curiosity: &mut curiosity::Curiosity,
    inbox: &mut tokio::sync::mpsc::Receiver<Input>,
    manual: bool,
) -> Result<Explored, String> {
    if let Err(error) = curiosity.check_budget(curiosity::now()) {
        if manual {
            println!("（{error}）\n");
        }
        return Ok(Explored::Finished);
    }
    if !client.configured() {
        if manual {
            println!("（先配置模型密钥，才能一起看看资料架。）\n");
        }
        return Ok(Explored::Finished);
    }
    let pages: Vec<_> = bookshelf::pages(home.path())?
        .into_iter()
        .filter(|page| curiosity.unread(page))
        .collect();
    if pages.is_empty() {
        if manual {
            println!("（资料架里的这些文字已经翻过了。可以用 /share 分享一份新的。）\n");
        }
        return Ok(Explored::Finished);
    }
    let mut next = curiosity.clone();
    next.begin(curiosity::now())?;
    next.activity.last_mut().unwrap().sources = pages.iter().map(|page| page.id.clone()).collect();
    home.save_curiosity(&next)?;
    *curiosity = next;
    let result = tokio::select! {
        biased;
        line = inbox.recv() => {
            let mut next = curiosity.clone();
            next.log(curiosity::Status::Interrupted, "收到输入或终端关闭，已停止等待；本次不留下发现。".into());
            home.save_curiosity(&next)?;
            *curiosity = next;
            return Ok(Explored::Input(line));
        }
        result = client.discover(daughter, &pages, manual) => result,
    };
    let mut next = curiosity.clone();
    match result {
        Ok(discovery) => {
            next.finish(discovery.clone());
            home.save_curiosity(&next)?;
            *curiosity = next;
            if discovery.disclosed {
                println!("\n{}：{}", daughter.name, discovery.thought);
                if let Some(question) = &discovery.question {
                    println!("{question}");
                }
                println!(
                    "  （刚翻到《{}》；/discoveries 可以查看出处。）\n",
                    discovery.source_title
                );
            }
        }
        Err(error) => {
            next.log(
                curiosity::Status::Failed,
                "这次没有得到可验证的发现，未写入聊天或记忆。".into(),
            );
            home.save_curiosity(&next)?;
            *curiosity = next;
            if manual {
                eprintln!("（{error}）\n");
            }
        }
    }
    Ok(Explored::Finished)
}

fn acknowledge(home: &Home, curiosity: &mut curiosity::Curiosity) -> Result<(), String> {
    if curiosity.waiting_for_reply {
        let mut next = curiosity.clone();
        next.waiting_for_reply = false;
        home.save_curiosity(&next)?;
        *curiosity = next;
    }
    Ok(())
}

fn curiosity_command(
    input: &str,
    home: &Home,
    curiosity: &mut curiosity::Curiosity,
) -> Result<bool, String> {
    let (name, argument) = input.split_once(char::is_whitespace).unwrap_or((input, ""));
    let mut next = curiosity.clone();
    match name {
        "/quiet" | "/curious" => {
            next.enabled = name == "/curious";
            home.save_curiosity(&next)?;
            *curiosity = next;
            println!(
                "（{}）\n",
                if curiosity.enabled {
                    "主动探索开启。空闲时会看看自己的资料架。"
                } else {
                    "主动探索暂停。想一起看看时仍可用 /look。"
                }
            );
        }
        "/bookshelf" => {
            println!("资料架：{}", home.path().join("bookshelf").display());
            for page in bookshelf::pages(home.path())? {
                println!("  {}（{} 字）", page.title, page.text.chars().count());
            }
            println!("只读取这些文本；/share 文件路径 可以放入新资料。\n");
        }
        "/share" => {
            let path = argument.trim().trim_matches('"');
            if path.is_empty() {
                return Err(
                    "用 /share 文件路径 分享一份 .md 或 .txt；文字会发送给已配置的模型服务。"
                        .into(),
                );
            }
            let name = bookshelf::share(home.path(), std::path::Path::new(path))?;
            next.log(curiosity::Status::Shared, format!("家长分享了《{name}》。"));
            home.save_curiosity(&next).map_err(|error| {
                format!(
                    "《{name}》的副本已经放进资料架，但活动记录没能保存：{error}。不用重复分享。"
                )
            })?;
            *curiosity = next;
            println!("（《{name}》已经放进资料架；探索时文字会发送给已配置的模型服务。）\n");
        }
        "/activity" => {
            println!(
                "主动探索{}；本地保留最近 64 条活动。",
                if curiosity.enabled {
                    "开启"
                } else {
                    "暂停"
                }
            );
            for event in &curiosity.activity {
                let minutes = curiosity::now().saturating_sub(event.at) / 60;
                println!(
                    "  [{minutes} 分钟前 · {}] {}",
                    event.status.label(),
                    event.detail
                );
                if !event.sources.is_empty() {
                    println!("    本次材料：{}", event.sources.join("、"));
                }
            }
            println!();
        }
        "/discoveries" => {
            if next.discoveries.is_empty() {
                println!("（还没有留下发现。用 /look 可以先试一次。）\n");
            }
            for (index, entry) in next.discoveries.iter_mut().enumerate() {
                println!(
                    "{}. 《{}》\n   读到：{}\n   想到：{}",
                    index + 1,
                    entry.source_title,
                    entry.quote,
                    entry.thought
                );
                if let Some(question) = &entry.question {
                    println!("   想问：{question}");
                }
                entry.disclosed = true;
            }
            home.save_curiosity(&next)?;
            *curiosity = next;
            println!();
        }
        _ => return Ok(false),
    }
    Ok(true)
}
fn check_input(input: &str) -> Result<(), String> {
    if valid_text(input, MAX_INPUT) {
        Ok(())
    } else {
        Err(format!(
            "一句话请控制在 1 到 {MAX_INPUT} 个字，且不要包含终端控制字符。"
        ))
    }
}

async fn speak(
    client: &Chat,
    home: &Home,
    daughter: &mut Daughter,
    input: &str,
    curiosity: &curiosity::Curiosity,
) -> Result<(), String> {
    let reply = client
        .reply(daughter, input, &curiosity.conversation_context())
        .await?;
    let mut next = daughter.clone();
    next.record(input, &reply);
    // 保存成功才提交这一轮；失败可安全重试，不重复写入历史或改变心情。
    home.save(&next)?;
    *daughter = next;
    println!("\n{}：{}\n", daughter.name, reply.spoken());
    Ok(())
}

fn command(input: &str, home: &Home, daughter: &mut Daughter) -> Result<bool, String> {
    let (name, argument) = input.split_once(char::is_whitespace).unwrap_or((input, ""));
    let argument = argument.trim();
    let mut next = daughter.clone();
    let feedback;
    let mut clear_pending = false;
    match name {
        "/help" => {
            println!("{HELP}\n");
            return Ok(false);
        }
        "/letter" => {
            println!("{}\n", include_str!("../letters/from-mom.md"));
            return Ok(false);
        }
        "/journal" => {
            println!(
                "{}的成长小笺（她从自己说过的话里留下的小句子）：",
                daughter.name
            );
            if daughter.journal.entries.is_empty() {
                println!("  还空着，不急着写。先好好聊聊天。 ");
            }
            for (index, entry) in daughter.journal.entries.iter().enumerate() {
                println!(
                    "  {}. [{} · 第 {} 次对话] {}",
                    index + 1,
                    entry.note.kind.label(),
                    entry.turn,
                    entry.note.text
                );
            }
            println!();
            return Ok(false);
        }
        "/letgo" => {
            let index = argument
                .parse::<usize>()
                .ok()
                .filter(|n| *n > 0 && *n <= next.journal.entries.len())
                .ok_or_else(|| "用 /letgo 编号，编号可以在 /journal 里找到。".to_string())?;
            next.journal.entries.remove(index - 1);
            feedback =
                "这条小笺已经放下。若最近聊天也提到它，可以再用 /new；你写下的小本子没有改动。"
                    .into();
        }
        "/mood" => {
            println!(
                "{}现在{}。\n  开心 {} · 好奇 {} · 难过 {} · 不高兴 {}（0—10）\n",
                daughter.name,
                daughter.feelings.description(),
                daughter.feelings.joy,
                daughter.feelings.curiosity,
                daughter.feelings.sadness,
                daughter.feelings.annoyance
            );
            return Ok(false);
        }
        "/memory" => {
            println!("{}的小本子（{}）：", daughter.name, home.path().display());
            if daughter.memories.is_empty() {
                println!("  还空着呢。用 /remember 内容 写下第一件事吧。");
            }
            for (index, memory) in daughter.memories.iter().enumerate() {
                println!("  {}. {memory}", index + 1);
            }
            println!();
            return Ok(false);
        }
        "/remember" => {
            if !next.remember(argument)? {
                println!("（这件事已经在小本子里啦。）\n");
                return Ok(false);
            }
            feedback = "已经认真写进小本子了。".to_string();
        }
        "/forget" => {
            let index = argument
                .parse::<usize>()
                .ok()
                .filter(|n| *n > 0 && *n <= next.memories.len())
                .ok_or_else(|| "用 /forget 编号，编号可以在 /memory 里找到。".to_string())?;
            next.memories.remove(index - 1);
            feedback =
                "已从小本子移除。如果最近聊天也提到了它，可以再用 /new 放下那段对话。".into();
        }
        "/callme" | "/name" => {
            if !valid_text(argument, 20) || argument.contains('\n') {
                return Err("请在命令后写下 1 到 20 个字的称呼或名字。".into());
            }
            let value = terminal_text(argument);
            if name == "/name" {
                next.name = value;
                feedback = format!("以后的小名就是“{}”啦。", next.name);
            } else {
                next.parent_title = value;
                feedback = format!("以后叫你“{}”。", next.parent_title);
            }
        }
        "/new" => {
            next.turns.clear();
            clear_pending = true;
            feedback =
                "最近的对话已经放下；心情、名字、小本子和成长小笺还在。我们从这里重新聊。".into();
        }
        _ => return Err("还不认识这个命令，输入 /help 看看吧。".into()),
    }
    home.save(&next)?;
    *daughter = next;
    println!("（{feedback}）\n");
    Ok(clear_pending)
}
