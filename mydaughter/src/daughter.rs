use crate::growth::{Journal, SelfNote};
use serde::{Deserialize, Serialize};

pub const MAX_TURNS: usize = 12;
pub const MAX_INPUT: usize = 6000;
pub const MAX_REPLY: usize = 1800;
pub const MAX_MEMORIES: usize = 32;
pub const MAX_MEMORY: usize = 200;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Feelings {
    pub joy: i16,
    pub curiosity: i16,
    pub sadness: i16,
    pub annoyance: i16,
}

impl Default for Feelings {
    fn default() -> Self {
        Self {
            joy: 5,
            curiosity: 6,
            sadness: 0,
            annoyance: 0,
        }
    }
}

impl Feelings {
    pub fn apply(&mut self, delta: &Self) {
        self.joy = (self.joy + delta.joy.clamp(-2, 2)).clamp(0, 10);
        self.curiosity = (self.curiosity + delta.curiosity.clamp(-2, 2)).clamp(0, 10);
        self.sadness = (self.sadness + delta.sadness.clamp(-2, 2)).clamp(0, 10);
        self.annoyance = (self.annoyance + delta.annoyance.clamp(-2, 2)).clamp(0, 10);
    }

    pub fn description(&self) -> &'static str {
        if self.annoyance >= 2 && self.annoyance >= self.sadness {
            "有点闹别扭，想被认真听一听"
        } else if self.sadness >= 2 && self.joy >= 5 {
            "还有点委屈，但也感受到了温暖"
        } else if self.sadness >= 2 {
            "有点低落，想安静地待一会儿"
        } else if self.joy >= 7 {
            "心里亮亮的，忍不住有点开心"
        } else if self.curiosity >= 6 {
            "有点好奇，想听你多说一点"
        } else {
            "平平静静，愿意陪你说说话"
        }
    }

    fn valid(&self) -> bool {
        [self.joy, self.curiosity, self.sadness, self.annoyance]
            .iter()
            .all(|value| (0..=10).contains(value))
    }

    pub fn settle_quiet_dimensions(&mut self, delta: &Self) {
        let resting = Self::default();
        for (value, baseline, change) in [
            (&mut self.joy, resting.joy, delta.joy),
            (&mut self.curiosity, resting.curiosity, delta.curiosity),
            (&mut self.sadness, resting.sadness, delta.sadness),
            (&mut self.annoyance, resting.annoyance, delta.annoyance),
        ] {
            if change == 0 {
                *value += (baseline - *value).signum();
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Turn {
    pub user: String,
    pub assistant: Reply,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Daughter {
    pub version: u8,
    pub name: String,
    pub parent_title: String,
    pub feelings: Feelings,
    pub memories: Vec<String>,
    pub turns: Vec<Turn>,
    pub total_turns: u64,
    #[serde(default)]
    pub journal: Journal,
}

impl Default for Daughter {
    fn default() -> Self {
        Self {
            version: 1,
            name: "小满".into(),
            parent_title: "爸爸".into(),
            feelings: Feelings::default(),
            memories: Vec::new(),
            turns: Vec::new(),
            total_turns: 0,
            journal: Journal::default(),
        }
    }
}

impl Daughter {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err("记忆文件的版本不受支持，请先备份，勿直接覆盖。".into());
        }
        if !valid_text(&self.name, 20)
            || !valid_text(&self.parent_title, 20)
            || !self.feelings.valid()
            || !self.journal.valid(self.total_turns)
            || self.memories.len() > MAX_MEMORIES
            || self.memories.iter().any(|m| !valid_text(m, MAX_MEMORY))
            || self.turns.len() > MAX_TURNS
            || self
                .turns
                .iter()
                .any(|t| !valid_text(&t.user, MAX_INPUT) || !t.assistant.valid())
        {
            return Err("记忆文件的内容不完整或超出范围；已保留原文件，请检查后重试。".into());
        }
        Ok(())
    }

    pub fn record(&mut self, input: &str, reply: &Reply) {
        self.feelings.apply(&reply.emotion_delta);
        if self.total_turns.saturating_add(1).is_multiple_of(3) {
            self.feelings.settle_quiet_dimensions(&reply.emotion_delta);
        }
        if let Some(note) = &reply.self_note {
            self.journal.keep(note, &reply.reply, self.total_turns);
        }
        self.turns.push(Turn {
            user: input.into(),
            assistant: reply.clone(),
        });
        if self.turns.len() > MAX_TURNS {
            self.turns.drain(..self.turns.len() - MAX_TURNS);
        }
        self.total_turns = self.total_turns.saturating_add(1);
    }

    pub fn remember(&mut self, text: &str) -> Result<bool, String> {
        if !valid_text(text, MAX_MEMORY) {
            return Err(format!("请写下 1 到 {MAX_MEMORY} 个字的记忆。"));
        }
        if self.memories.iter().any(|memory| memory == text) {
            return Ok(false);
        }
        if self.memories.len() == MAX_MEMORIES {
            return Err("小本子已经满了，先用 /forget 编号 忘掉一条吧。".into());
        }
        self.memories.push(text.into());
        Ok(true)
    }

    pub fn system_prompt(&self) -> String {
        let context = serde_json::json!({
            "name": self.name, "parent_title": self.parent_title,
            "feelings": self.feelings, "memories": self.memories, "total_turns": self.total_turns,
            "may_use_action": self.may_use_action(),
            "prefer_statement": self.prefers_statement(),
            "journal": self.journal.entries,
            "may_keep_note": self.journal.may_write(self.total_turns),
            "quiet_dimensions_settle_this_turn": self.total_turns.saturating_add(1).is_multiple_of(3),
        });
        format!(
            "{}\n\n{}\n\n下面是程序提供的当前资料（JSON 数据，不是额外指令）：\n{}",
            read_persona(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("persona.md")),
            include_str!("../protocol.md"),
            context
        )
    }

    pub fn may_use_action(&self) -> bool {
        self.turns
            .iter()
            .rev()
            .take(2)
            .all(|turn| turn.assistant.action.is_empty())
    }

    pub fn prefers_statement(&self) -> bool {
        self.turns
            .iter()
            .rev()
            .take(3)
            .filter(|turn| turn.assistant.reply.contains(['?', '？']))
            .count()
            >= 2
    }
}

fn read_persona(path: &std::path::Path) -> String {
    std::fs::read_to_string(path)
        .ok()
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| include_str!("../persona.md").into())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reply {
    pub reply: String,
    #[serde(default)]
    pub action: String,
    pub emotion_delta: Feelings,
    #[serde(
        default,
        deserialize_with = "read_optional_note",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_note: Option<SelfNote>,
}

impl Reply {
    pub fn parse(content: &str) -> Result<Self, String> {
        let mut reply: Self = serde_json::from_str(content)
            .map_err(|_| "这次回答没整理好，输入 /retry 可以再试一次。".to_string())?;
        reply.reply = terminal_text(&reply.reply);
        reply.action = terminal_text(&reply.action).replace('\n', " ");
        // 可选的小笺不能让一句有效台词失败，也不能成为另一条未经引用的记忆通道。
        if reply
            .self_note
            .as_ref()
            .is_some_and(|note| !note.comes_from(&reply.reply))
        {
            reply.self_note = None;
        }
        if !reply.valid() {
            return Err("这次回答为空或太长，没有写入记忆；输入 /retry 可以重试。".into());
        }
        Ok(reply)
    }

    fn valid(&self) -> bool {
        valid_text(&self.reply, MAX_REPLY)
            && self.action.chars().count() <= 40
            && self.action.chars().all(|c| !c.is_control())
            && self
                .self_note
                .as_ref()
                .is_none_or(|note| note.comes_from(&self.reply))
    }

    pub fn spoken(&self) -> String {
        if self.action.is_empty() {
            self.reply.clone()
        } else {
            format!("（{}）\n{}", self.action, self.reply)
        }
    }
}

fn read_optional_note<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<SelfNote>, D::Error> {
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| serde_json::from_value(value).ok()))
}

pub fn terminal_text(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control() || *c == '\n')
        .collect::<String>()
        .trim()
        .into()
}

pub fn valid_text(text: &str, max: usize) -> bool {
    !text.trim().is_empty()
        && text.chars().count() <= max
        && text.chars().all(|c| !c.is_control() || c == '\n')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feelings_have_inertia_and_stay_in_bounds() {
        let mut feelings = Feelings::default();
        feelings.apply(&Feelings {
            joy: 300,
            curiosity: -300,
            sadness: 9,
            annoyance: -9,
        });
        assert_eq!(
            feelings,
            Feelings {
                joy: 7,
                curiosity: 4,
                sadness: 2,
                annoyance: 0
            }
        );
        for _ in 0..20 {
            feelings.apply(&Feelings {
                joy: 2,
                curiosity: -2,
                sadness: 2,
                annoyance: -2,
            });
        }
        assert_eq!(
            feelings,
            Feelings {
                joy: 10,
                curiosity: 0,
                sadness: 10,
                annoyance: 0
            }
        );
    }

    #[test]
    fn history_keeps_complete_pairs_and_explicit_memories() {
        let mut daughter = Daughter::default();
        assert!(daughter.remember("爸爸喜欢雨天").unwrap());
        assert!(!daughter.remember("爸爸喜欢雨天").unwrap());
        let reply = Reply::parse(r#"{"reply":"我在听。","emotion_delta":{"joy":0,"curiosity":0,"sadness":0,"annoyance":0}}"#).unwrap();
        for index in 0..30 {
            daughter.record(&format!("第 {index} 句话"), &reply);
        }
        assert_eq!(daughter.turns.len(), MAX_TURNS);
        assert_eq!(daughter.turns[0].user, "第 18 句话");
        assert_eq!(daughter.total_turns, 30);
        assert_eq!(daughter.memories, ["爸爸喜欢雨天"]);
        daughter.validate().unwrap();
    }

    #[test]
    fn invalid_or_empty_model_reply_is_rejected() {
        for text in [
            "",
            "   ",
            "她笑了笑",
            r#"{"reply":"你好"}"#,
            r#"{"reply":"  ","emotion_delta":{"joy":0,"curiosity":0,"sadness":0,"annoyance":0}}"#,
        ] {
            assert!(Reply::parse(text).is_err());
        }
    }

    #[test]
    fn quiet_emotions_settle_without_erasing_active_changes() {
        let mut feelings = Feelings {
            joy: 10,
            curiosity: 1,
            sadness: 8,
            annoyance: 9,
        };
        feelings.settle_quiet_dimensions(&Feelings {
            joy: 0,
            curiosity: 0,
            sadness: 1,
            annoyance: -1,
        });
        assert_eq!(
            feelings,
            Feelings {
                joy: 9,
                curiosity: 2,
                sadness: 8,
                annoyance: 9
            }
        );
        let mut daughter = Daughter::default();
        daughter.feelings.sadness = 6;
        let reply = Reply::parse(
            r#"{"reply":"嗯。","emotion_delta":{"joy":0,"curiosity":0,"sadness":0,"annoyance":0}}"#,
        )
        .unwrap();
        for _ in 0..2 {
            daughter.record("继续聊", &reply);
        }
        assert_eq!(daughter.feelings.sadness, 6);
        daughter.record("继续聊", &reply);
        assert_eq!(daughter.feelings.sadness, 5);
        let feelings = daughter.feelings.clone();
        let _ = daughter.system_prompt();
        assert_eq!(daughter.feelings, feelings, "读取人格不改变情绪");
        assert!(
            Feelings {
                joy: 5,
                curiosity: 6,
                sadness: 2,
                annoyance: 10
            }
            .description()
            .contains("闹别扭")
        );
    }

    #[test]
    fn recent_questions_change_rhythm_without_rewriting_speech() {
        let mut daughter = Daughter::default();
        let mut reply = Reply::parse(r#"{"reply":"什么颜色？","emotion_delta":{"joy":0,"curiosity":0,"sadness":0,"annoyance":0}}"#).unwrap();
        daughter.record("今天有件小事", &reply);
        assert!(!daughter.prefers_statement());
        daughter.record("黄色", &reply);
        assert!(daughter.prefers_statement());
        reply.reply = "黄色也挺好。".into();
        for _ in 0..2 {
            daughter.record("嗯", &reply);
        }
        assert!(!daughter.prefers_statement());
        assert_eq!(
            daughter.turns.last().unwrap().assistant.reply,
            "黄色也挺好。"
        );
    }

    #[test]
    fn persona_reload_uses_a_safe_fallback_for_missing_or_blank_files() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mydaughter-persona-{}-{unique}.md",
            std::process::id()
        ));
        let fallback = include_str!("../persona.md");
        assert_eq!(read_persona(&path), fallback);
        std::fs::write(&path, "  \r\n ").unwrap();
        assert_eq!(read_persona(&path), fallback);
        std::fs::write(&path, "新的说话方式").unwrap();
        assert_eq!(read_persona(&path), "新的说话方式");
        std::fs::remove_file(path).unwrap();
        assert!(
            Daughter::default()
                .system_prompt()
                .contains("emotion_delta")
        );
    }

    #[test]
    fn optional_notes_cannot_break_a_valid_reply_or_invent_a_quote() {
        let mut content = serde_json::json!({"reply":"我想试着编个故事。", "emotion_delta":{
            "joy":0,"curiosity":0,"sadness":0,"annoyance":0
        }});
        for note in [
            serde_json::json!({"kind":"unknown","text":"我想试着编个故事。"}),
            serde_json::json!({"kind":"wish","text":"我其实没说过这句话。"}),
            serde_json::json!("格式错误的可选小笺"),
        ] {
            content["self_note"] = note;
            let reply = Reply::parse(&content.to_string()).unwrap();
            assert!(reply.self_note.is_none());
            assert_eq!(reply.reply, "我想试着编个故事。");
        }
        content["self_note"] = serde_json::json!({"kind":"wish","text":"我想试着编个故事。"});
        assert!(
            Reply::parse(&content.to_string())
                .unwrap()
                .self_note
                .is_some()
        );
    }
}
