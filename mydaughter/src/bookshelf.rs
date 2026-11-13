//! 自主阅读只接触资料架的顶层文本。模型只会得到 Page，不会得到文件操作能力。
//! 路径检查可防常见越界、链接和误分享；这不是抵御本机恶意并发替换的 OS 隔离层。

use serde::Serialize;
use std::{
    fs::{self, File, Metadata, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

const MAX_BYTES: u64 = 32 * 1024;
const MAX_PAGE_CHARS: usize = 4000;
const MAX_TOTAL_CHARS: usize = 12000;
const MAX_USER_PAGES: usize = 24;
const MAX_DIRECTORY_ENTRIES: usize = 256;

#[derive(Clone, Debug, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub text: String,
}

/// 不创建目录、不递归。缺少资料架时仍有三张内置小纸条可读。
pub fn pages(home: &Path) -> Result<Vec<Page>, String> {
    let mut result = starter_pages();
    let Some(root) = shelf_root(home, false)? else {
        return Ok(result);
    };
    let entries = fs::read_dir(&root).map_err(|_| "暂时打不开资料架目录。".to_string())?;
    let mut paths: Vec<PathBuf> = entries
        .take(MAX_DIRECTORY_ENTRIES)
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| allowed_name(path))
        .collect();
    paths.sort();
    let remaining = MAX_TOTAL_CHARS - result.iter().map(|p| p.text.chars().count()).sum::<usize>();
    let mut candidates = Vec::new();
    for path in paths {
        if candidates.len() == MAX_USER_PAGES {
            break;
        }
        // 有问题的单个文件不阻断其余资料；不将错误正文或文件内容写到终端。
        let Ok(text) = read_text(&path, Some(&root)) else {
            continue;
        };
        let text: String = clean_text(&text).chars().take(MAX_PAGE_CHARS).collect();
        if text.trim().is_empty() {
            continue;
        }
        let title = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        candidates.push(Page {
            id: format!("file:{title}"),
            title,
            text,
        });
    }
    // 先确定合格候选，再平均分配文字预算，避免排序靠前的大文件挡住后面的资料。
    let allowance = (remaining / candidates.len().max(1)).min(MAX_PAGE_CHARS);
    for mut page in candidates {
        page.text = page.text.chars().take(allowance).collect();
        result.push(page);
    }
    Ok(result)
}

/// 只能由用户明确的 /share 调用；复制文件，不修改源，不覆盖资料架中的同名文件。
pub fn share(home: &Path, source: &Path) -> Result<String, String> {
    if !allowed_name(source) {
        return Err(
            "只接收普通 .md 或 .txt 文件；隐藏文件和带密钥、口令等敏感名称的文件不能分享。".into(),
        );
    }
    let source = checked_absolute(source)?;
    let text = read_text(&source, None)?;
    if text.trim().is_empty() {
        return Err("这份文字是空的，先放一点内容再分享吧。".into());
    }
    // 先完成源文件校验，再允许创建专用资料架目录。
    let root = shelf_root(home, true)?.ok_or_else(|| "资料架目录没有创建成功。".to_string())?;
    let name = source
        .file_name()
        .ok_or_else(|| "找不到文件名。".to_string())?;
    let destination = root.join(name);
    check_no_links(&root)?;
    if root
        .canonicalize()
        .map_err(|_| "资料架位置发生了变化。".to_string())?
        != root
    {
        return Err("资料架位置发生了变化，请重新分享。".into());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&destination)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                "资料架里已有同名文件，没有覆盖。给新文件换个名字再分享吧。".into()
            } else {
                "没能把这份文字放进资料架，请检查目录权限。".to_string()
            }
        })?;
    check_contained(&destination, &root)?;
    file.write_all(text.as_bytes())
        .and_then(|_| file.sync_all())
        .map_err(|_| "这次没有完整写入资料架；源文件未改动，请检查新副本。".to_string())?;
    check_contained(&destination, &root)?;
    Ok(name.to_string_lossy().into_owned())
}

fn starter_pages() -> Vec<Page> {
    [
        ("builtin:small-names", "给普通东西起名字", "一张原创小纸条：给一盏普通的小灯起名字，可以先想它的颜色、样子，或者它让你想到的一个词。名字不必解释所有东西，也可以只是叫着顺口。试着给同一盏灯起一个认真的名字，再起一个有点傻的名字。你会留下哪一个？"),
        ("builtin:unfinished-story", "一个还没写完的小故事", "一个想象里的故事开头：一把伞说它想休假，鞋子却说今天正适合出门。它们都还没解释自己的理由。你可以接着编一句，也可以挑一个角色问问它。这个故事是虚构的，没有真的发生在谁的家里。"),
        ("builtin:ordinary-words", "把一句普通话翻过来看看", "一张原创观察小纸条：‘随便’有时候是在说哪种都行，有时候只是还没想好。‘等一下’也可能很短，可能很长。这里没有标准答案。挑一个你觉得有意思的普通词，想一个具体的小场景，看看同一句话换个人说会不会变味。"),
    ].into_iter().map(|(id, title, text)| Page { id: id.into(), title: title.into(), text: text.into() }).collect()
}

fn allowed_name(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    if name.starts_with('.')
        || name.chars().count() > 120
        || name
            .chars()
            .any(|c| c.is_control() || is_bidi_control(c) || c == ':')
    {
        return false;
    }
    let lowered = name.to_lowercase();
    if [
        ".env",
        "credential",
        "password",
        "secret",
        "private_key",
        "private-key",
        "token",
        "api_key",
        "api-key",
        "apikey",
        "id_rsa",
        "id_ed25519",
        "密钥",
        "口令",
        "密码",
        "凭证",
    ]
    .iter()
    .any(|word| lowered.contains(word))
    {
        return false;
    }
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("md") || extension.eq_ignore_ascii_case("txt")
        })
}

fn checked_absolute(path: &Path) -> Result<PathBuf, String> {
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err("资料路径不能包含 ..，请使用明确的文件位置。".into());
    }
    #[cfg(windows)]
    if path.components().any(|component| matches!(component, Component::Prefix(prefix)
        if !matches!(prefix.kind(), std::path::Prefix::Disk(_) | std::path::Prefix::VerbatimDisk(_)))) {
        return Err("资料架只接收本机磁盘文件，不接收网络或设备路径。".into());
    }
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|_| "找不到当前目录。".to_string())?
            .join(path)
    };
    if !absolute.is_absolute() {
        return Err("请使用完整的文件路径。".into());
    }
    check_no_links(&absolute)?;
    Ok(absolute)
}

fn check_no_links(path: &Path) -> Result<(), String> {
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(metadata) if is_link(&metadata) => {
                return Err("资料路径包含链接或重定向目录，暂不读取。".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err("无法检查资料路径，请检查文件权限。".into()),
        }
    }
    Ok(())
}

fn is_link(metadata: &Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        // Windows junction、符号链接和其他 reparse point 都不进入。
        metadata.file_type().is_symlink() || metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        metadata.file_type().is_symlink()
    }
}

fn shelf_root(home: &Path, create: bool) -> Result<Option<PathBuf>, String> {
    let home = checked_absolute(home)?;
    if create {
        fs::create_dir_all(&home).map_err(|_| "不能创建小满的目录。".to_string())?;
    }
    check_no_links(&home)?;
    let home = match home.canonicalize() {
        Ok(path) => path,
        Err(error) if !create && error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("找不到小满的目录。".into()),
    };
    let shelf = home.join("bookshelf");
    check_no_links(&shelf)?;
    if create {
        match fs::create_dir(&shelf) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => return Err("不能创建资料架。".into()),
        }
    }
    let root = match shelf.canonicalize() {
        Ok(path) => path,
        Err(error) if !create && error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("找不到资料架目录。".into()),
    };
    check_no_links(&shelf)?;
    if root.parent() != Some(home.as_path()) || !root.is_dir() {
        return Err("资料架必须位于小满自己的目录里。".into());
    }
    Ok(Some(root))
}

fn check_contained(path: &Path, root: &Path) -> Result<PathBuf, String> {
    check_no_links(path)?;
    let resolved = path
        .canonicalize()
        .map_err(|_| "资料文件的位置发生了变化。".to_string())?;
    if resolved.parent() != Some(root) || root.canonicalize().ok().as_deref() != Some(root) {
        return Err("这份资料不在授权资料架的顶层。".into());
    }
    Ok(resolved)
}

fn read_text(path: &Path, root: Option<&Path>) -> Result<String, String> {
    let absolute = checked_absolute(path)?;
    let resolved = match root {
        Some(root) => check_contained(&absolute, root)?,
        None => absolute
            .canonicalize()
            .map_err(|_| "找不到这份文字文件。".to_string())?,
    };
    let before =
        fs::symlink_metadata(&absolute).map_err(|_| "不能检查这份文字文件。".to_string())?;
    if !before.is_file() || is_link(&before) || before.len() > MAX_BYTES {
        return Err("只读取不超过 32 KiB 的普通文字文件。".into());
    }
    let file = File::open(&absolute).map_err(|_| "不能打开这份文字文件。".to_string())?;
    let opened = file
        .metadata()
        .map_err(|_| "不能检查已经打开的文件。".to_string())?;
    if !opened.is_file() || is_link(&opened) || opened.len() > MAX_BYTES {
        return Err("这份文件的类型或大小不适合放进资料架。".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "这次没有读完资料文件。".to_string())?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("文件超过 32 KiB，没有读取更多内容。".into());
    }
    check_no_links(&absolute)?;
    let after_path = match root {
        Some(root) => check_contained(&absolute, root)?,
        None => absolute
            .canonicalize()
            .map_err(|_| "读取时文件的位置发生了变化。".to_string())?,
    };
    let after =
        fs::symlink_metadata(&absolute).map_err(|_| "读取时文件发生了变化。".to_string())?;
    if after_path != resolved
        || !after.is_file()
        || is_link(&after)
        || before.len() != opened.len()
        || opened.len() != after.len()
        || before.modified().ok() != after.modified().ok()
    {
        return Err("这份文件在读取时发生了变化，请稍后再试。".into());
    }
    String::from_utf8(bytes).map_err(|_| "资料需要使用 UTF-8 编码。".to_string())
}

fn clean_text(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_control() || is_bidi_control(c) {
                ' '
            } else {
                c
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDir(PathBuf);
    impl TestDir {
        fn new() -> Self {
            let stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "mydaughter-bookshelf-{}-{stamp}",
                std::process::id()
            ));
            fs::create_dir(&path).unwrap();
            Self(path.canonicalize().unwrap())
        }
    }
    impl Drop for TestDir {
        fn drop(&mut self) {
            // 只处理本测试创建的、已确定的临时目录。
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_new_home_has_three_starters_without_filesystem_writes() {
        let temp = TestDir::new();
        let home = temp.0.join("not-created");
        let pages = pages(&home).unwrap();
        assert_eq!(pages.len(), 3);
        assert!(pages.iter().all(|page| page.id.starts_with("builtin:")));
        assert!(!home.exists());
    }

    #[test]
    fn only_top_level_approved_text_is_read_and_sizes_are_bounded() {
        let temp = TestDir::new();
        let shelf = temp.0.join("bookshelf");
        fs::create_dir(&shelf).unwrap();
        fs::create_dir(shelf.join("nested")).unwrap();
        fs::write(shelf.join("nested/hidden.txt"), "不能递归看到").unwrap();
        for name in [
            ".env.txt",
            "my_password.md",
            "TOKEN.txt",
            "private_key.md",
            "账号密码.txt",
            "服务密钥.md",
            "登录口令.txt",
            "访问凭证.md",
            "script.ps1",
        ] {
            fs::write(shelf.join(name), "不能读取的内容").unwrap();
        }
        fs::write(shelf.join("invalid.txt"), [0xff, 0xfe]).unwrap();
        fs::write(
            shelf.join("too-big.txt"),
            vec![b'a'; MAX_BYTES as usize + 1],
        )
        .unwrap();
        fs::write(shelf.join("allowed.md"), "正常\n内容\u{1b}\u{202e}").unwrap();
        fs::write(shelf.join("long.txt"), "长".repeat(5000)).unwrap();
        let result = pages(&temp.0).unwrap();
        assert_eq!(result.len(), 5);
        let allowed = result
            .iter()
            .find(|page| page.title == "allowed.md")
            .unwrap();
        assert!(!allowed.text.chars().any(char::is_control));
        assert!(!allowed.text.contains('\u{202e}'));
        assert_eq!(
            result
                .iter()
                .find(|page| page.title == "long.txt")
                .unwrap()
                .text
                .chars()
                .count(),
            MAX_PAGE_CHARS
        );
    }

    #[test]
    fn all_pages_together_fit_the_total_budget() {
        let temp = TestDir::new();
        let shelf = temp.0.join("bookshelf");
        fs::create_dir(&shelf).unwrap();
        for index in 0..30 {
            fs::write(shelf.join(format!("page-{index}.md")), "字".repeat(4000)).unwrap();
        }
        let result = pages(&temp.0).unwrap();
        assert!(result.len() <= MAX_USER_PAGES + 3);
        assert!(
            result
                .iter()
                .map(|page| page.text.chars().count())
                .sum::<usize>()
                <= MAX_TOTAL_CHARS
        );
    }

    #[test]
    fn long_names_and_obvious_chinese_secrets_are_rejected() {
        assert!(allowed_name(Path::new(&format!("{}.md", "字".repeat(117)))));
        assert!(!allowed_name(Path::new(&format!(
            "{}.md",
            "字".repeat(118)
        ))));
        for name in ["旧密钥.txt", "管理员口令.md", "我的密码.txt", "测试凭证.md"] {
            assert!(!allowed_name(Path::new(name)));
        }
    }

    #[test]
    fn all_twenty_four_long_files_remain_visible() {
        let temp = TestDir::new();
        let shelf = temp.0.join("bookshelf");
        fs::create_dir(&shelf).unwrap();
        for index in 0..MAX_USER_PAGES {
            fs::write(shelf.join(format!("long-{index:02}.md")), "长".repeat(5000)).unwrap();
        }
        let result = pages(&temp.0).unwrap();
        assert_eq!(result.len(), MAX_USER_PAGES + 3);
        for index in 0..MAX_USER_PAGES {
            let page = result
                .iter()
                .find(|page| page.title == format!("long-{index:02}.md"))
                .unwrap();
            assert!(
                page.text.chars().count() >= 450,
                "每份长资料都应拿到可读的文字预算"
            );
            assert!(page.text.chars().count() <= MAX_PAGE_CHARS);
        }
        assert!(
            result
                .iter()
                .map(|page| page.text.chars().count())
                .sum::<usize>()
                <= MAX_TOTAL_CHARS
        );
    }

    #[test]
    fn sharing_preserves_the_source_and_never_overwrites() {
        let temp = TestDir::new();
        let home = temp.0.join("home");
        let source = temp.0.join("shared.md");
        fs::write(&source, "一份明确分享的文字\n保留原来的换行。\n").unwrap();
        assert_eq!(share(&home, &source).unwrap(), "shared.md");
        assert_eq!(
            fs::read_to_string(&source).unwrap(),
            "一份明确分享的文字\n保留原来的换行。\n"
        );
        fs::write(&source, "第二次的文字").unwrap();
        assert!(share(&home, &source).is_err());
        assert_eq!(
            fs::read_to_string(home.join("bookshelf/shared.md")).unwrap(),
            "一份明确分享的文字\n保留原来的换行。\n"
        );
        let secret = temp.0.join("credentials.txt");
        fs::write(&secret, "不应分享").unwrap();
        assert!(share(&home, &secret).is_err());
        assert!(!home.join("bookshelf/credentials.txt").exists());
    }

    #[test]
    fn sibling_and_parent_paths_are_not_inside_the_shelf() {
        let temp = TestDir::new();
        fs::create_dir(temp.0.join("bookshelf")).unwrap();
        let root = temp.0.join("bookshelf").canonicalize().unwrap();
        let outside = temp.0.join("outside.txt");
        fs::write(&outside, "不在资料架里").unwrap();
        assert!(read_text(&outside, Some(&root)).is_err());
        assert!(read_text(&root.join("../outside.txt"), Some(&root)).is_err());
        assert!(share(&temp.0, Path::new("../outside.txt")).is_err());
    }

    #[test]
    fn linked_files_and_linked_shelves_are_not_followed() {
        let temp = TestDir::new();
        let home = temp.0.join("home");
        fs::create_dir(&home).unwrap();
        fs::create_dir(home.join("bookshelf")).unwrap();
        let source = temp.0.join("outside.txt");
        fs::write(&source, "链接指向的外部文字").unwrap();
        let link = home.join("bookshelf/linked.txt");
        #[cfg(unix)]
        let link_result = std::os::unix::fs::symlink(&source, &link);
        #[cfg(windows)]
        let link_result = std::os::windows::fs::symlink_file(&source, &link);
        if let Err(error) = link_result {
            #[cfg(windows)]
            if error.raw_os_error() == Some(1314) {
                eprintln!(
                    "Windows 未授予创建符号链接的权限，跳过实际链接夹具；其余路径测试仍运行。"
                );
                return;
            }
            panic!("创建测试链接失败：{error}");
        }
        assert_eq!(pages(&home).unwrap().len(), 3);
        assert!(share(&home, &link).is_err());
        fs::remove_file(&link).unwrap();
        fs::remove_dir(home.join("bookshelf")).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&temp.0, home.join("bookshelf")).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&temp.0, home.join("bookshelf")).unwrap();
        assert!(pages(&home).is_err());
        assert!(share(&home, &source).is_err());
        #[cfg(unix)]
        fs::remove_file(home.join("bookshelf")).unwrap();
        #[cfg(windows)]
        fs::remove_dir(home.join("bookshelf")).unwrap();
    }
}
