use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BibtexFormattingOptions {
    pub line_length: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct LatexFormattingOptions {
    pub latexindent: Option<LatexIndentOptions>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct LatexIndentOptions {
    pub executable: Option<String>,
    pub args: Option<Vec<String>>,
}

impl LatexIndentOptions {
    pub fn executable(&self) -> String {
        self.executable
            .as_ref()
            .map(Clone::clone)
            .unwrap_or_else(|| "latexindent".into())
    }

    pub fn args(&self) -> Vec<String> {
        self.args
            .as_ref()
            .map(Clone::clone)
            .unwrap_or_else(|| vec!["-c".into(), "%d".into(), "%f".into()])
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct LatexForwardSearchOptions {
    pub executable: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LatexLintOptions {
    pub on_change: Option<bool>,
    pub on_save: Option<bool>,
}

impl LatexLintOptions {
    pub fn on_change(&self) -> bool {
        self.on_change.unwrap_or(false)
    }

    pub fn on_save(&self) -> bool {
        self.on_save.unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatexBuildOptions {
    pub executable: Option<String>,
    pub args: Option<Vec<String>>,
    pub on_save: Option<bool>,
}

impl LatexBuildOptions {
    pub fn executable(&self) -> String {
        self.executable
            .as_ref()
            .map(Clone::clone)
            .unwrap_or_else(|| "latexmk".into())
    }

    pub fn args(&self) -> Vec<String> {
        self.args.as_ref().map(Clone::clone).unwrap_or_else(|| {
            vec![
                "-pdf".into(),
                "-interaction=nonstopmode".into(),
                "-synctex=1".into(),
            ]
        })
    }

    pub fn on_save(&self) -> bool {
        self.on_save.unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatexOptions {
    pub forward_search: Option<LatexForwardSearchOptions>,
    pub formatting: Option<LatexFormattingOptions>,
    pub lint: Option<LatexLintOptions>,
    pub build: Option<LatexBuildOptions>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BibtexOptions {
    pub formatting: Option<BibtexFormattingOptions>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub latex: Option<LatexOptions>,
    pub bibtex: Option<BibtexOptions>,
}
