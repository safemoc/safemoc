use crate::{
    bookshelf::Page,
    daughter::{terminal_text, valid_text},
};
use serde::{Deserialize, Serialize};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub const IDLE_SECONDS: u64 = 120;
pub const COOLDOWN_SECONDS: u64 = 300;
const MAX_DISCOVERIES: usize = 32;
const MAX_EVENTS: usize = 64;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Discovery {
    pub at: u64,
    pub source_id: String,
    pub source_title: String,
    pub fingerprint: String,
    pub quote: String,
    pub thought: String,
    pub question: Option<String>,
    pub disclosed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Started,
    Completed,
    Interrupted,
    Failed,
    Shared,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Started => "已开始",
            Self::Completed => "已读完",
            Self::Interrupted => "已停下",
            Self::Failed => "未完成",
            Self::Shared => "收到分享",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub at: u64,
    pub status: Status,
    pub detail: String,
    #[serde(default)]
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Curiosity {
    pub version: u8,
    pub enabled: bool,
    pub waiting_for_reply: bool,
    pub attempts: Vec<u64>,
    pub discoveries: Vec<Discovery>,
    pub activity: Vec<Activity>,
    #[serde(skip)]
    session_attempts: u8,
    #[serde(skip)]
    last_started: Option<Instant>,
}

impl Default for Curiosity {
    fn default() -> Self {
        Self {
            version: 1,
            enabled: true,
            waiting_for_reply: false,
            attempts: Vec::new(),
            discoveries: Vec::new(),
            activity: Vec::new(),
            session_attempts: 0,
            last_started: None,
        }
    }
}

impl Curiosity {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1
            || self.attempts.len() > 6
            || self.discoveries.len() > MAX_DISCOVERIES
            || self.activity.len() > MAX_EVENTS
            || self.activity.iter().any(|event| {
                !valid_text(&event.detail, 300)
                    || event.sources.len() > 27
                    || event.sources.iter().any(|source| !valid_text(source, 160))
            })
            || self.discoveries.iter().any(|entry| {
                !valid_text(&entry.source_id, 160)
                    || !valid_text(&entry.source_title, 160)
                    || !valid_text(&entry.fingerprint, 32)
                    || !valid_text(&entry.quote, 240)
                    || !valid_text(&entry.thought, 240)
                    || entry.question.as_ref().is_some_and(|q| !valid_text(q, 160))
            })
        {
            return Err("探索记录内容不完整；原文件已保留，请检查 curiosity.json。".into());
        }
        Ok(())
    }

    pub fn check_budget(&self, at: u64) -> Result<(), String> {
        if self.waiting_for_reply {
            return Err("还有一条发现想和你聊，先继续说话或看看 /discoveries 吧。".into());
        }
        if self.session_attempts >= 3 {
            return Err("这次打开已经探索过三次了，先聊聊已有的小发现吧。".into());
        }
        if self
            .attempts
            .iter()
            .filter(|time| at.saturating_sub(**time) < 3600)
            .count()
            >= 6
        {
            return Err("这一小时的探索次数已经用完，晚点再看。".into());
        }
        if self
            .attempts
            .last()
            .is_some_and(|time| at.saturating_sub(*time) < COOLDOWN_SECONDS)
            || self
                .last_started
                .is_some_and(|time| time.elapsed() < Duration::from_secs(COOLDOWN_SECONDS))
        {
            return Err("刚探索过，休息五分钟再看下一页。".into());
        }
        Ok(())
    }

    pub fn begin(&mut self, at: u64) -> Result<(), String> {
        self.check_budget(at)?;
        self.attempts.retain(|time| at.saturating_sub(*time) < 3600);
        self.attempts.push(at);
        self.session_attempts += 1;
        self.last_started = Some(Instant::now());
        self.log(Status::Started, "在资料架里找一点感兴趣的东西。".into());
        Ok(())
    }

    pub fn log(&mut self, status: Status, detail: String) {
        self.activity.push(Activity {
            at: now(),
            status,
            detail,
            sources: Vec::new(),
        });
        if self.activity.len() > MAX_EVENTS {
            self.activity.remove(0);
        }
    }

    pub fn unread(&self, page: &Page) -> bool {
        let fingerprint = fingerprint(page);
        !self
            .discoveries
            .iter()
            .any(|discovery| discovery.source_id == page.id && discovery.fingerprint == fingerprint)
    }

    pub fn finish(&mut self, discovery: Discovery) {
        self.waiting_for_reply = discovery.question.is_some();
        self.log(
            Status::Completed,
            format!("读了《{}》，留下了一点发现。", discovery.source_title),
        );
        self.discoveries.push(discovery);
        if self.discoveries.len() > MAX_DISCOVERIES {
            self.discoveries.remove(0);
        }
    }

    pub fn conversation_context(&self) -> Vec<Discovery> {
        let mut recent: Vec<_> = self.discoveries.iter().rev().take(2).cloned().collect();
        recent.reverse();
        recent
    }
}

fn fingerprint(page: &Page) -> String {
    let mut hash = DefaultHasher::new();
    page.text.hash(&mut hash);
    format!("{:016x}", hash.finish())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Proposal {
    source_id: String,
    quote: String,
    thought: String,
    #[serde(default)]
    question: Option<String>,
}

pub fn parse_discovery(content: &str, pages: &[Page], manual: bool) -> Result<Discovery, String> {
    let proposal: Proposal =
        serde_json::from_str(content).map_err(|_| "探索回答格式没有整理好。")?;
    let page = pages
        .iter()
        .find(|page| page.id == proposal.source_id)
        .ok_or("探索来源不在本次资料架里。")?;
    let quote = proposal.quote.trim().to_string();
    let thought = terminal_text(&proposal.thought);
    let question = proposal
        .question
        .map(|q| terminal_text(&q))
        .filter(|q| !q.is_empty());
    if !valid_text(&quote, 240)
        || !page.text.contains(&quote)
        || !valid_text(&thought, 240)
        || question.as_ref().is_some_and(|q| !valid_text(q, 160))
    {
        return Err("探索引用或小发现没有通过检查。".into());
    }
    Ok(Discovery {
        at: now(),
        source_id: page.id.clone(),
        source_title: page.title.clone(),
        fingerprint: fingerprint(page),
        quote,
        thought,
        disclosed: manual || question.is_some(),
        question,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn page() -> Page {
        Page {
            id: "paper-1".into(),
            title: "小纸条".into(),
            text: "给一朵云起个名字。".into(),
        }
    }

    #[test]
    fn a_model_cannot_invent_sources_quotes_or_operations() {
        let pages = [page()];
        for content in [
            r#"{"source_id":"../../.env","quote":"给一朵云","thought":"想看看。"}"#,
            r#"{"source_id":"paper-1","quote":"并不存在","thought":"想看看。"}"#,
            r#"{"source_id":"paper-1","quote":"给一朵云","thought":"想看看。","command":"delete"}"#,
        ] {
            assert!(parse_discovery(content, &pages, false).is_err());
        }
        let discovery = parse_discovery(r#"{"source_id":"paper-1","quote":"给一朵云","thought":"我想叫它慢慢。","question":null}"#, &pages, false).unwrap();
        assert!(!discovery.disclosed);
        let mut curiosity = Curiosity::default();
        curiosity.finish(discovery);
        assert!(!curiosity.unread(&pages[0]));
        assert!(
            !curiosity.conversation_context()[0].disclosed,
            "自己的发现可供接话，但不能伪装成已向家长说过"
        );
        let mut changed = page();
        changed.text.push('呀');
        assert!(curiosity.unread(&changed));
    }

    #[test]
    fn budget_survives_restart_failure_and_clock_rollback() {
        let mut curiosity = Curiosity::default();
        curiosity.begin(10000).unwrap();
        curiosity.log(Status::Interrupted, "收到新消息，先停下。".into());
        let loaded: Curiosity =
            serde_json::from_str(&serde_json::to_string(&curiosity).unwrap()).unwrap();
        assert!(loaded.check_budget(10001).is_err());
        assert!(loaded.check_budget(9000).is_err());
        let mut hourly = loaded.clone();
        hourly.attempts = vec![10000; 6];
        assert!(hourly.check_budget(11000).is_err());
        assert!(hourly.check_budget(14000).is_ok());
        curiosity.session_attempts = 3;
        assert!(curiosity.check_budget(14000).is_err());
    }
}
