use human_format::{Formatter, Scales};

#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Deserialize, Serialize)]
        #[allow(dead_code)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub fn fmt_num(num: i32) -> String {
    let decimals = if num > 999 { 1 } else { 0 };
    let mut scales = Scales::new();
    scales.with_base(1000).with_suffixes(vec!["", "k", "M"]);

    Formatter::new()
        .with_scales(scales)
        .with_decimals(decimals)
        .with_separator("")
        .format(num as f64)
}

pub fn fmt_dur(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;

    match (h, m, s) {
        (h, m, _) if h > 0 => format!("{h}h {m}m"),
        (_, m, _) if m > 0 => format!("{m}m {s}s"),
        _ => format!("{s}s"),
    }
}
