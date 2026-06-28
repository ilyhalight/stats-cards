use serde::{Deserialize, Serialize};

pub enum ActivityColor {
    Inactive,
    Small,
    Medium,
    High,
    VeryHigh,
}

impl ActivityColor {
    pub fn from_key(key: &str) -> Option<ActivityColor> {
        match key {
            "#ebedf0" => Some(ActivityColor::Inactive),
            "#9be9a8" => Some(ActivityColor::Small),
            "#40c463" => Some(ActivityColor::Medium),
            "#30a14e" => Some(ActivityColor::High),
            "#216e39" => Some(ActivityColor::VeryHigh),
            _ => None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ThemeData {
    pub background: String,
    pub surface_background: String,
    pub text: String,
    pub header: String,
    pub mono_icon: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Theme {
    #[serde(rename = "catppuccin-macchiato", alias = "catpuccin-macchiato")]
    CatppuccinMacchiato,
    #[serde(rename = "catppuccin-mocha")]
    CatppuccinMocha,
    #[serde(rename = "catppuccin-latte")]
    CatppuccinLatte,
    #[serde(rename = "catppuccin-frappe")]
    CatppuccinFrappe,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "onedark-pro-flat")]
    OneDarkProFlat,
    #[serde(rename = "dracula")]
    Dracula,
    #[serde(rename = "kanagawa-wave")]
    KanagawaWave,
    #[serde(rename = "ayu-mirage")]
    AyuMirage,
    #[serde(rename = "ayu-white")]
    AyuWhite,
    #[serde(rename = "monokai-classic")]
    MonokaiClassic,
    #[serde(rename = "black-pink")]
    BlackPink,
}

impl Theme {
    pub fn get_data(&self) -> ThemeData {
        match self {
            // #region Catppuccin (https://catppuccin.com/palette/)
            Theme::CatppuccinMacchiato => ThemeData {
                // base
                background: "#24273A".to_string(),
                // surface 0
                surface_background: "#363a4f".to_string(),
                // text
                text: "#CAD3F5".to_string(),
                // mauve
                header: "#C6A0F6".to_string(),
                // overlay 1
                mono_icon: "#8087a2".to_string(),
            },
            Theme::CatppuccinMocha => ThemeData {
                // base
                background: "#1e1e2e".to_string(),
                // surface 0
                surface_background: "#313244".to_string(),
                // text
                text: "#cdd6f4".to_string(),
                // mauve
                header: "#cba6f7".to_string(),
                // overlay 1
                mono_icon: "#7f849c".to_string(),
            },
            Theme::CatppuccinLatte => ThemeData {
                // base
                background: "#eff1f5".to_string(),
                // surface 0
                surface_background: "#ccd0da".to_string(),
                // text
                text: "#4c4f69".to_string(),
                // mauve
                header: "#8839ef".to_string(),
                // overlay 1
                mono_icon: "#8c8fa1".to_string(),
            },
            Theme::CatppuccinFrappe => ThemeData {
                // base
                background: "#303446".to_string(),
                // surface 0
                surface_background: "#414559".to_string(),
                // text
                text: "#c6d0f5".to_string(),
                // mauve
                header: "#ca9ee6".to_string(),
                // overlay 1
                mono_icon: "#838ba7".to_string(),
            },
            // #endregion Catppuccin
            Theme::Dark => ThemeData {
                background: "#1D1D1D".to_string(),
                surface_background: "#27272A".to_string(),
                text: "#cfcfcf".to_string(),
                header: "#FF6363".to_string(),
                mono_icon: "#A5A5A5".to_string(),
            },
            Theme::White => ThemeData {
                background: "#fff".to_string(),
                surface_background: "#e5e5e5".to_string(),
                text: "#05010d".to_string(),
                header: "#FF6363".to_string(),
                mono_icon: "#A5A5A5".to_string(),
            },
            // OneDark Pro https://github.com/Binaryify/OneDark-Pro
            Theme::OneDarkProFlat => ThemeData {
                // editor.background
                background: "#282c34".to_string(),
                // button.background
                surface_background: "#404754".to_string(),
                // activityBar.foreground
                text: "#c7ccd6".to_string(),
                // js variable readwrite
                header: "#e06c75".to_string(),
                // editorInlayHint.foreground
                mono_icon: "#abb2bf".to_string(),
            },
            // Dracula https://draculatheme.com/contribute
            Theme::Dracula => ThemeData {
                // background
                background: "#282A36".to_string(),
                // current line (selection)
                surface_background: "#44475A".to_string(),
                // foreground
                text: "#F8F8F2".to_string(),
                // cyan
                header: "#8be9fd".to_string(),
                // color-checks-btn-icon (in github theme)
                mono_icon: "#8b949e".to_string(),
            },
            // Kanagawa Wave https://github.com/rebelot/kanagawa.nvim
            Theme::KanagawaWave => ThemeData {
                // sumlink1
                background: "#1F1F28".to_string(),
                // sumiInk3
                surface_background: "#363646".to_string(),
                // fujiWhite
                text: "#DCD7BA".to_string(),
                // carpYellow
                header: "#E6C384".to_string(),
                // fujiGray
                mono_icon: "#727169".to_string(),
            },
            // #region Ayu https://github.com/ayu-theme/ayu-colors
            Theme::AyuMirage => ThemeData {
                // editor.bg
                background: "#242936".to_string(),
                // ui.selection.normal
                surface_background: "#69758C1F".to_string(),
                // editor.fg
                text: "#CCCAC2".to_string(),
                // common.accent
                header: "#FFCC66".to_string(),
                // editor.gutter.active
                mono_icon: "#8A9199CC".to_string(),
            },
            Theme::AyuWhite => ThemeData {
                // editor.bg
                background: "#FCFCFC".to_string(),
                // ui.selection.normal
                surface_background: "#6B7D8F1F".to_string(),
                // editor.fg
                text: "#5C6166".to_string(),
                // common.accent
                header: "#FFAA33".to_string(),
                // editor.gutter.active
                mono_icon: "#8A9199CC".to_string(),
            },
            // #endregion Ayu
            // Monokai Classic https://github.com/microsoft/vscode/blob/main/extensions/theme-monokai/themes/monokai-color-theme.json
            Theme::MonokaiClassic => ThemeData {
                // editor background
                background: "#272822".to_string(),
                // dropdown.background
                surface_background: "#414339".to_string(),
                // editor foreground
                text: "#f8f8f2".to_string(),
                // String
                header: "#E6DB74".to_string(),
                // focus
                mono_icon: "#75715e".to_string(),
            },
            // by maestroTW
            // based on bear theme from https://github.com/anuraghazra/github-readme-stats/tree/master/themes
            Theme::BlackPink => ThemeData {
                background: "#1f2023".to_string(),       
                surface_background: "#343434".to_string(), 
                text: "#beb698".to_string(),             
                header: "#c8397d".to_string(),            
                mono_icon: "#847c64".to_string(),         
            },
        }
    }

    pub fn get_activity_color(&self, activity_color: ActivityColor) -> String {
        match self {
            Theme::CatppuccinMacchiato => match activity_color {
                // surface 1
                ActivityColor::Inactive => "#494d64".to_string(),
                ActivityColor::Small => "#42583c".to_string(),
                ActivityColor::Medium => "#7ea072".to_string(),
                // green
                ActivityColor::High => "#a6da95".to_string(),
                ActivityColor::VeryHigh => "#8ddb73".to_string(),
            },
            Theme::CatppuccinMocha => match activity_color {
                // surface 1
                ActivityColor::Inactive => "#45475a".to_string(),
                ActivityColor::Small => "#42583c".to_string(),
                ActivityColor::Medium => "#7ea072".to_string(),
                // green
                ActivityColor::High => "#a6e3a1".to_string(),
                ActivityColor::VeryHigh => "#8ddb73".to_string(),
            },
            Theme::CatppuccinLatte => match activity_color {
                // surface 1
                ActivityColor::Inactive => "#bcc0cc".to_string(),
                ActivityColor::Small => "#9be9a8".to_string(),
                ActivityColor::Medium => "#40c463".to_string(),
                // green
                ActivityColor::High => "#40a02b".to_string(),
                ActivityColor::VeryHigh => "#216e39".to_string(),
            },
            Theme::CatppuccinFrappe => match activity_color {
                // surface 1
                ActivityColor::Inactive => "#51576d".to_string(),
                ActivityColor::Small => "#42583c".to_string(),
                ActivityColor::Medium => "#7ea072".to_string(),
                // green
                ActivityColor::High => "#a6d189".to_string(),
                ActivityColor::VeryHigh => "#8ddb73".to_string(),
            },
            Theme::Dark => match activity_color {
                ActivityColor::Inactive => "#27272A".to_string(),
                ActivityColor::Small => "#42583c".to_string(),
                ActivityColor::Medium => "#7ea072".to_string(),
                ActivityColor::High => "#a6da95".to_string(),
                ActivityColor::VeryHigh => "#8ddb73".to_string(),
            },
            Theme::White => match activity_color {
                ActivityColor::Inactive => "#e5e5e5".to_string(),
                ActivityColor::Small => "#9be9a8".to_string(),
                ActivityColor::Medium => "#40c463".to_string(),
                ActivityColor::High => "#30a14e".to_string(),
                ActivityColor::VeryHigh => "#216e39".to_string(),
            },
            Theme::OneDarkProFlat => match activity_color {
                // button.background
                ActivityColor::Inactive => "#404754".to_string(),
                ActivityColor::Small => "#42583c".to_string(),
                ActivityColor::Medium => "#7ea072".to_string(),
                // Strings
                ActivityColor::High => "#98c379".to_string(),
                ActivityColor::VeryHigh => "#8ddb73".to_string(),
            },
            // based on https://github.com/dracula/github
            Theme::Dracula => match activity_color {
                // current line (selection)
                ActivityColor::Inactive => "#44475A".to_string(),
                // comment
                ActivityColor::Small => "#6272a4".to_string(),
                // cyan
                ActivityColor::Medium => "#8be9fd".to_string(),
                // purple
                ActivityColor::High => "#bd93f9".to_string(),
                // pink
                ActivityColor::VeryHigh => "#ff79c6".to_string(),
            },
            Theme::KanagawaWave => match activity_color {
                // sumiInk3
                ActivityColor::Inactive => "#363646".to_string(),
                // middle value from winterYellow and boatYellow1
                ActivityColor::Small => "#7B6B45".to_string(),
                // boatYellow2
                ActivityColor::Medium => "#C0A36E".to_string(),
                // autumnYellow
                ActivityColor::High => "#DCA561".to_string(),
                // carpYellow
                ActivityColor::VeryHigh => "#E6C384".to_string(),
            },
            Theme::AyuMirage => match activity_color {
                // editor.indentGuide.active
                ActivityColor::Inactive => "#8A919959".to_string(),
                // syntax.func + remove some contrast
                ActivityColor::Small => "#EACA88".to_string(),
                // syntax.operator
                ActivityColor::Medium => "#F29E74".to_string(),
                // vcs.removed
                ActivityColor::High => "#F27983".to_string(),
                // common.error
                ActivityColor::VeryHigh => "#FF6666".to_string(),
            },
            Theme::AyuWhite => match activity_color {
                // editor.indentGuide.active
                ActivityColor::Inactive => "#8A919959".to_string(),
                // syntax.func + remove some contrast
                ActivityColor::Small => "#F4C989".to_string(),
                // syntax.operator
                ActivityColor::Medium => "#ED9366".to_string(),
                // vcs.removed
                ActivityColor::High => "#FF7383".to_string(),
                // common.error
                ActivityColor::VeryHigh => "#E65050".to_string(),
            },
            Theme::MonokaiClassic => match activity_color {
                // editorHoverWidget.border
                ActivityColor::Inactive => "#75715E".to_string(),
                // inputValidation.warningBackground
                ActivityColor::Small => "#848528".to_string(),
                // String
                ActivityColor::Medium => "#E6DB74".to_string(),
                // inputValidation.warningBorder
                ActivityColor::High => "#e2e22e".to_string(),
                // Class name
                ActivityColor::VeryHigh => "#A6E22E".to_string(),
            },
            // by maestroTW
            // based on bear theme from https://github.com/anuraghazra/github-readme-stats/tree/master/themes
            Theme::BlackPink => match activity_color {
                ActivityColor::Inactive => "#343434".to_string(),    
                ActivityColor::Small => "#604a4c".to_string(),       
                ActivityColor::Medium => "#847c64".to_string(),       
                ActivityColor::High => "#c8397d".to_string(),    
                ActivityColor::VeryHigh => "#42283b".to_string(),
            },
        }
    }
}


