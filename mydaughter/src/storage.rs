use crate::{curiosity::Curiosity, daughter::Daughter};
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static SAVE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
const MAX_CURIOSITY_BYTES: u64 = 256 * 1024;

pub struct SessionLock {
    // File 关闭即释放 OS 锁；保留锁文件，避免 unlink 后另一个实例锁到不同文件。
    _file: File,
}

pub struct Home {
    path: PathBuf,
}

impl Home {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn lock(&self) -> Result<SessionLock, String> {
        self.prepare_directory()?;
        let path = self.path.join("session.lock");
        check_plain_path(&path)?;
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            // 允许其他实例打开后尝试锁，但持锁期间不允许删除或替换锁文件。
            options.share_mode(0x0000_0001 | 0x0000_0002);
        }
        let file = options
            .open(&path)
            .map_err(|e| format!("打不开会话锁：{e}"))?;
        check_plain_path(&path)?;
        if !file.metadata().map_err(|e| e.to_string())?.is_file() {
            return Err("会话锁必须是普通文件。".into());
        }
        match file.try_lock() {
            Ok(()) => Ok(SessionLock { _file: file }),
            Err(TryLockError::WouldBlock) => Err("这个月亮窝已经在另一个窗口里打开了。请先退出那个会话，或使用不同的 MYDAUGHTER_HOME。".into()),
            Err(TryLockError::Error(error)) => Err(format!("无法锁定月亮窝：{error}")),
        }
    }

    pub fn load(&self) -> Result<Daughter, String> {
        let path = self.path.join("daughter.json");
        check_plain_path(&path)?;
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Daughter::default()),
            Err(error) => return Err(format!("读不了记忆文件 {}：{error}", path.display())),
        };
        let daughter: Daughter = serde_json::from_slice(&bytes).map_err(|_| {
            format!(
                "记忆文件 {} 已损坏；原文件已保留，请先备份并检查。",
                path.display()
            )
        })?;
        daughter.validate()?;
        Ok(daughter)
    }

    pub fn save(&self, daughter: &Daughter) -> Result<(), String> {
        daughter.validate()?;
        let bytes = serde_json::to_vec_pretty(daughter).map_err(|e| e.to_string())?;
        self.write_atomic("daughter.json", &bytes)
    }

    pub fn load_curiosity(&self) -> Result<Curiosity, String> {
        let path = self.path.join("curiosity.json");
        check_plain_path(&path)?;
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(Curiosity::default());
            }
            Err(error) => return Err(format!("读不了探索记录 {}：{error}", path.display())),
        };
        if !file.metadata().map_err(|e| e.to_string())?.is_file() {
            return Err("探索记录必须是普通文件。".into());
        }
        let mut bytes = Vec::new();
        file.take(MAX_CURIOSITY_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("读取探索记录失败：{e}"))?;
        if bytes.len() as u64 > MAX_CURIOSITY_BYTES {
            return Err("探索记录超过 256 KiB，原文件已保留，请先检查。".into());
        }
        check_plain_path(&path)?;
        let curiosity: Curiosity = serde_json::from_slice(&bytes).map_err(|_| {
            format!(
                "探索记录 {} 已损坏；原文件已保留，请先检查。",
                path.display()
            )
        })?;
        curiosity.validate()?;
        Ok(curiosity)
    }

    pub fn save_curiosity(&self, curiosity: &Curiosity) -> Result<(), String> {
        curiosity.validate()?;
        let bytes = serde_json::to_vec_pretty(curiosity).map_err(|e| e.to_string())?;
        if bytes.len() as u64 > MAX_CURIOSITY_BYTES {
            return Err("探索记录超过 256 KiB，没有覆盖原文件。".into());
        }
        self.write_atomic("curiosity.json", &bytes)
    }

    fn prepare_directory(&self) -> Result<(), String> {
        check_plain_path(&self.path)?;
        fs::create_dir_all(&self.path).map_err(|e| format!("创建记忆目录失败：{e}"))?;
        check_plain_path(&self.path)
    }

    fn write_atomic(&self, name: &str, bytes: &[u8]) -> Result<(), String> {
        self.prepare_directory()?;
        let destination = self.path.join(name);
        check_plain_path(&destination)?;
        // 同目录临时文件 + rename，避免中断时把现有记忆截断。
        let temporary = self.path.join(format!(
            "{name}.{}.{}.tmp",
            std::process::id(),
            SAVE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let mut created = false;
        let result = (|| -> io::Result<()> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            created = true;
            file.write_all(bytes)?;
            file.sync_all()?;
            drop(file);
            check_plain_path(&destination).map_err(io::Error::other)?;
            fs::rename(&temporary, destination)
        })();
        if let Err(error) = result {
            if created {
                let _ = fs::remove_file(&temporary);
            }
            return Err(format!(
                "记忆没有保存成功：{error}。原有记忆未改变，请检查目录 {} 后重试。",
                self.path.display()
            ));
        }
        Ok(())
    }
}

fn check_plain_path(path: &Path) -> Result<(), String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(path)
    };
    for part in absolute.ancestors().collect::<Vec<_>>().into_iter().rev() {
        let metadata = match fs::symlink_metadata(part) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(format!("检查月亮窝路径失败：{error}")),
        };
        let is_link = metadata.file_type().is_symlink();
        #[cfg(windows)]
        let is_link = {
            use std::os::windows::fs::MetadataExt;
            is_link || metadata.file_attributes() & 0x0000_0400 != 0
        };
        if is_link {
            return Err(format!(
                "月亮窝路径不能经过符号链接或重解析点：{}",
                part.display()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_home(label: &str) -> Home {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Home::new(std::env::temp_dir().join(format!(
            "mydaughter-{label}-{}-{unique}",
            std::process::id()
        )))
    }

    #[test]
    fn session_lock_excludes_another_handle_and_releases_on_drop() {
        let home = test_home("lock");
        let first = home.lock().unwrap();
        assert!(home.lock().is_err());
        drop(first);
        let second = home.lock().unwrap();
        drop(second);
        assert!(home.path.join("session.lock").is_file());
        fs::remove_file(home.path.join("session.lock")).unwrap();
        fs::remove_dir(home.path()).unwrap();
    }

    #[test]
    fn curiosity_roundtrip_and_failed_reads_preserve_original() {
        let home = test_home("curiosity");
        let curiosity = home.load_curiosity().unwrap();
        home.save_curiosity(&curiosity).unwrap();
        assert_eq!(
            serde_json::to_value(home.load_curiosity().unwrap()).unwrap(),
            serde_json::to_value(&curiosity).unwrap()
        );
        let path = home.path.join("curiosity.json");
        fs::write(&path, b"broken").unwrap();
        assert!(home.load_curiosity().is_err());
        assert_eq!(fs::read(&path).unwrap(), b"broken");
        fs::write(&path, vec![b' '; MAX_CURIOSITY_BYTES as usize + 1]).unwrap();
        assert!(home.load_curiosity().is_err());
        assert_eq!(fs::metadata(&path).unwrap().len(), MAX_CURIOSITY_BYTES + 1);
        fs::remove_file(path).unwrap();
        fs::remove_dir(home.path()).unwrap();
    }

    #[test]
    fn save_replace_reload_and_preserve_corruption() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("mydaughter-test-{}-{unique}", std::process::id()));
        let home = Home::new(path.clone());
        let mut daughter = home.load().unwrap();
        home.save(&daughter).unwrap();
        daughter.remember("一起给窗口起名叫月亮窝").unwrap();
        daughter.feelings.sadness = 3;
        home.save(&daughter).unwrap();
        let loaded = home.load().unwrap();
        assert_eq!(loaded.memories, daughter.memories);
        assert_eq!(loaded.feelings, daughter.feelings);
        let file = path.join("daughter.json");
        fs::write(&file, "broken").unwrap();
        assert!(home.load().is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), "broken");
        fs::remove_file(file).unwrap();
        fs::remove_dir(path).unwrap();
    }
}
