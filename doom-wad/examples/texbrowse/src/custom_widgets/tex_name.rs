use std::borrow::Cow;
use regex::{Regex, Captures as Caps};
use lazy_static::lazy_static;

const MIN_BASE_CHARS: usize = 7;
const MAX_BASE_CHARS: usize = 10;
const MAX_DIR_LEVELS: usize = 2;

#[derive(Debug, Clone)]
pub struct TextureName<'a>(pub Cow<'a, str>);
impl<'a> TextureName<'a> {
    pub fn short_name(&self) -> String {
        lazy_static! {
        static ref TEXNAME_NAME_R: Regex = Regex::new(r"/([^/]+)$").unwrap();
        static ref TEXNAME_FOLDER_R: Regex = Regex::new(r"([^/]{2,})/").unwrap();
        static ref TEXNAME_FOLDERS_R: Regex = Regex::new(r"([^/]{1,2}/)").unwrap();
        }
        let full_path = self.0.contains('/');
        if !full_path {
            if self.0.len() > 8 {
                format!("{}..", &self.0[0..8])
            } else {
                format!("{}", self.0)
            }
        } else {
            // STEP: Shorten directory names to one character and a dot
            let text = TEXNAME_FOLDER_R.replace_all(&self.0, |caps: &Caps| {
                let dir_name_len = caps[1].len();
                let folder_first = (&caps[1]).chars().next().unwrap();
                let mut dst = String::new();
                dst.push(folder_first);
                if dir_name_len > 1 {
                    dst.push('.');
                }
                dst.push('/');
                dst
            });
            // STEP: Shorten pointlessly long file names
            let text = TEXNAME_NAME_R.replace(&text, |caps: &Caps| {
                let cap = &caps[1];
                let mut extn = String::new();
                let basename_len =
                if let Some((base, ext)) = cap.rsplit_once('.') {
                    extn = ext.to_string();
                    base.len()
                } else {
                    cap.len()
                };
                if basename_len > MAX_BASE_CHARS {
                    format!("/{}..{}", &cap[0..MIN_BASE_CHARS], extn)
                } else {
                    format!("/{}", cap)
                }
            });
            // STEP: Shorten directories if there are more than three
            let mut subdir_count = TEXNAME_FOLDERS_R.find_iter(&text).count();
            if subdir_count > MAX_DIR_LEVELS {
                subdir_count -= 1;
                let text = TEXNAME_FOLDERS_R.replacen(&text, subdir_count, "");
                format!("..{}", text)
            } else {
                text.to_string()
            }
        }
    }
}

impl<'a, T> From<T> for TextureName<'a>
    where T: Into<Cow<'a, str>> {
    fn from(v: T) -> Self {
        TextureName(v.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_name_short() {
        let tex_name = "STUD3_5".to_string();
        let tex_name = TextureName(Cow::from(&tex_name));
        assert_eq!(tex_name.short_name(), "STUD3_5".to_string())
    }

    #[test]
    fn texture_name_long() {
        let tex_name = "LONGABILLABONGA".to_string();
        let tex_name = TextureName(Cow::from(&tex_name));
        assert_eq!(tex_name.short_name(), "LONGABIL..".to_string())
    }

    #[test]
    fn texture_name_full_path() {
        let tex_name = "textures/studs/stud3_5.png".to_string();
        let tex_name = TextureName(Cow::from(&tex_name));
        assert_eq!(tex_name.short_name(), "t./s./stud3_5.png".to_string())
    }

    #[test]
    fn texture_name_stupidly_long() {
        let tex_name = "textures/studs/this_is_a_stupidly_and_pointlessly_long_texture_name_why_did_you_call_your_texture_this_stupidly_and_pointlessly_long_name_and_just_how_insane_do_you_have_to_be_to_do_something_like_this.png".to_string();
        let tex_name = TextureName(Cow::from(&tex_name));
        assert_eq!(tex_name.short_name(), "t./s./this_is..png".to_string())
    }

    #[test]
    fn texture_name_subdirs() {
        let tex_name = "textures/studs/alpha/beta/crap/deck.png".to_string();
        let tex_name = TextureName(Cow::from(&tex_name));
        assert_eq!(tex_name.short_name(), "..c./deck.png".to_string())
    }

    #[test]
    fn texture_name_subdirs2() {
        let tex_name = "textures/studs/alpha/beta/c/deck.png".to_string();
        let tex_name = TextureName(Cow::from(&tex_name));
        assert_eq!(tex_name.short_name(), "..c/deck.png".to_string())
    }
}
