#[derive(Debug)]
pub enum CppCompiler {
    Clang,
    Gcc,
}

impl CppCompiler {
    pub fn new(s: &str) -> Self {
        match s {
            "Clang" => Self::Clang,
            "Gcc" => Self::Gcc,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Lang {
    Cpp(u8, CppCompiler, String),
    C(u8, CppCompiler, String),
    Rust,
    Python(u8),
    PyPy(u8),
    Js,
    Go,
    Ruby,
    Java,
    Pascal(String),
}

impl std::convert::From<&json::JsonValue> for Lang {
    fn from(val: &json::JsonValue) -> Self {
        let name = val["name"].as_str().unwrap();
        let version = val["version"].as_u8();
        let compiler = val["compiler"].as_str();
        let optimize_level = val["optimize_level"].as_str();

        match name {
            "C++" => Self::Cpp(
                version.unwrap(),
                CppCompiler::new(compiler.unwrap()),
                optimize_level.unwrap().to_string(),
            ),
            "C" => Self::C(
                version.unwrap(),
                CppCompiler::new(compiler.unwrap()),
                optimize_level.unwrap().to_string(),
            ),
            "Python" => Self::Python(version.unwrap()),
            "PyPy" => Self::PyPy(version.unwrap()),
            "Rust" => Self::Rust,
            "Js" => Self::Js,
            "Go" => Self::Go,
            "Ruby" => Self::Ruby,
            "Java" => Self::Java,
            "Pascal" => Self::Pascal(optimize_level.unwrap().to_string()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Code {
    pub src: String,
    pub lang: Lang,
}

impl std::convert::From<&json::JsonValue> for Code {
    fn from(val: &json::JsonValue) -> Self {
        Self {
            src: val["src"].as_str().unwrap().to_string(),
            lang: Lang::from(&val["lang"]),
        }
    }
}

impl Code {
    pub fn new(src: String, lang: Lang) -> Self {
        Self {
            src: src,
            lang: lang,
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub pid: i32,
    pub sid: u32,
    pub code: Code,
}

impl Task {
    pub fn new(pid: i32, sid: u32, code: Code) -> Self {
        Self {
            pid: pid,
            sid: sid,
            code: code,
        }
    }

    pub async fn fetch(uri: &String) -> Self {
        let resp = reqwest::get(uri)
            .await
            .unwrap()
            .text_with_charset("utf-8")
            .await
            .unwrap();

        let parsed = json::parse(&resp).unwrap();

        Self {
            pid: parsed["pid"].as_i32().unwrap(),
            sid: parsed["sid"].as_u32().unwrap(),
            code: Code::from(&parsed["code"]),
        }
    }
}
