use crate::err::Error;
use regex::Regex;

#[derive(Clone, Debug)]
pub enum VersionKind {
    Develop,
    Alpha,
    Beta,
    Preview,
    Release,
}

impl VersionKind {
    fn new(s: &str) -> Option<VersionKind> {
        if s.eq("a") || s.eq("alpha") {
            return Some(VersionKind::Alpha);
        }
        if s.eq("b") || s.eq("beta") {
            return Some(VersionKind::Beta);
        }

        if s.eq("r") || s.eq("release") {
            return Some(VersionKind::Release);
        }

        if s.eq("p") || s.eq("pre") || s.eq("preview") {
            return Some(VersionKind::Preview);
        }

        if s.eq("d") || s.eq("dev") || s.eq("devel") || s.eq("develop") || s.eq("development") {
            return Some(VersionKind::Develop);
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    kind: VersionKind,
}

pub type VersionStore = u32;

impl Version {
    pub fn from(s: &str) -> Option<Version> {
        let s = s.trim();
        let r = Regex::new(r"v(\d+)\.(\d+)\.(\d+)\s?(?:(?:\((\w+)\))|(?:\-\s?(\w+)))");
        let r = r.expect("Error when creating a new regex pattern");
        let cap = r.captures(s);
        if cap.is_none() {
            return None;
        }
        let cap = cap.unwrap();
        let major = cap.get(1);
        let minor = cap.get(2);
        let patch = cap.get(3);
        let mut kind = cap.get(4);
        if kind.is_none() {
            kind = cap.get(5);
        }

        if major.is_none() || minor.is_none() || patch.is_none() || kind.is_none() {
            return None;
        }

        let kind = VersionKind::new(kind.unwrap().as_str());
        if kind.is_none() {
            return None;
        }

        let major = cap.get(1).unwrap().as_str().parse::<u8>();
        let minor = cap.get(2).unwrap().as_str().parse::<u8>();
        let patch = cap.get(3).unwrap().as_str().parse::<u8>();

        if major.is_err() || minor.is_err() || patch.is_err() {
            return None;
        }

        Some(Version {
            major: major.unwrap(),
            minor: minor.unwrap(),
            patch: patch.unwrap(),
            kind: kind.unwrap(),
        })
    }

    pub fn from_store(vs: VersionStore) -> Option<Version> {
        let u = vs as u32;
        let kind: VersionKind;
        match (u & 0x000000FF >> 0) as u8 {
            0x0 => kind = VersionKind::Develop,
            0x1 => kind = VersionKind::Alpha,
            0x2 => kind = VersionKind::Beta,
            0x3 => kind = VersionKind::Preview,
            0x4 => kind = VersionKind::Release,
            _ => return None,
        }
        Some(Version {
            major: ((u & 0xFF000000) >> 24) as u8,
            minor: ((u & 0x00FF0000) >> 16) as u8,
            patch: ((u & 0x0000FF00) >> 8) as u8,
            kind,
        })
    }

    pub fn to_store(self) -> VersionStore {
        let kind: u32;
        match self.kind {
            VersionKind::Develop => kind = 0x0,
            VersionKind::Alpha => kind = 0x1,
            VersionKind::Beta => kind = 0x2,
            VersionKind::Preview => kind = 0x3,
            VersionKind::Release => kind = 0x4,
        };

        let major = self.major as u32;
        let minor = self.minor as u32;
        let patch = self.patch as u32;

        (major << 24 | minor << 16 | patch << 8 | kind) as VersionStore
    }
}
