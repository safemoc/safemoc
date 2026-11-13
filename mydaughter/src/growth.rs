use crate::daughter::{MAX_REPLY, valid_text};
use serde::{Deserialize, Serialize};

pub const MAX_NOTES: usize = 16;
const MAX_NOTE: usize = 120;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Preference,
    Wish,
    Reflection,
}

impl NoteKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Preference => "小偏爱",
            Self::Wish => "小愿望",
            Self::Reflection => "慢慢懂得",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SelfNote {
    pub kind: NoteKind,
    pub text: String,
}

impl SelfNote {
    pub fn valid(&self) -> bool {
        valid_text(&self.text, MAX_NOTE) && !self.text.contains('\n')
    }

    pub fn comes_from(&self, reply: &str) -> bool {
        self.valid() && reply.chars().count() <= MAX_REPLY && reply.contains(&self.text)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JournalEntry {
    pub note: SelfNote,
    pub turn: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Journal {
    pub entries: Vec<JournalEntry>,
    // 删除小笺不会绕过书写间隔；只记录成功对话的序号，不用离线时间计分。
    pub last_written_turn: u64,
}

impl Journal {
    pub fn may_write(&self, total_turns: u64) -> bool {
        total_turns >= 2
            && self.entries.len() < MAX_NOTES
            && (self.last_written_turn == 0
                || total_turns.saturating_sub(self.last_written_turn) >= 3)
    }

    pub fn keep(&mut self, note: &SelfNote, reply: &str, total_turns: u64) {
        if !self.may_write(total_turns)
            || !note.comes_from(reply)
            || self
                .entries
                .iter()
                .any(|entry| entry.note.text == note.text)
        {
            return;
        }
        let turn = total_turns.saturating_add(1);
        self.entries.push(JournalEntry {
            note: note.clone(),
            turn,
        });
        self.last_written_turn = turn;
    }

    pub fn valid(&self, total_turns: u64) -> bool {
        self.entries.len() <= MAX_NOTES
            && self.last_written_turn <= total_turns
            && self.entries.iter().all(|entry| {
                entry.note.valid() && entry.turn > 0 && entry.turn <= self.last_written_turn
            })
            && self
                .entries
                .windows(2)
                .all(|entries| entries[0].turn < entries[1].turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notes_require_actual_spoken_words_and_space_to_grow() {
        let note = SelfNote {
            kind: NoteKind::Wish,
            text: "我想试着编一个没有坏人的故事。".into(),
        };
        let mut journal = Journal::default();
        journal.keep(&note, &note.text, 0);
        assert!(journal.entries.is_empty());
        journal.keep(&note, "我没有这样说。", 2);
        assert!(journal.entries.is_empty());
        journal.keep(&note, &note.text, 2);
        assert_eq!(journal.entries.len(), 1);
        assert_eq!(journal.entries[0].turn, 3);
        assert!(!journal.may_write(5));
        assert!(journal.may_write(6));
        journal.keep(&note, &note.text, 6);
        assert_eq!(journal.entries.len(), 1, "不重复收藏同一句");
        journal.entries.clear();
        assert!(!journal.may_write(3), "删除也保留原有间隔");
    }

    #[test]
    fn full_notebook_preserves_earlier_entries() {
        let mut journal = Journal::default();
        for index in 0..MAX_NOTES + 2 {
            let note = SelfNote {
                kind: NoteKind::Reflection,
                text: format!("我想留下第 {index} 个念头。"),
            };
            journal.keep(&note, &note.text, (index * 4 + 2) as u64);
        }
        assert_eq!(journal.entries.len(), MAX_NOTES);
        assert_eq!(journal.entries[0].note.text, "我想留下第 0 个念头。");
        assert!(journal.valid(100));
    }
}
