pub mod date
{
    pub fn str_to_seconds(s: &str) -> u64
    {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        let mut total_seconds = 0;
        for s in split
        {
            if let Some(chr) = s.chars().last()
            {
                match chr
                {
                    'm' =>
                    {
                        let s = s.replace("m", "");
                        if let Ok(m) = s.parse::<u64>()
                        {
                            total_seconds += m * 60;
                        }
                    }
                    's' =>
                    {
                        let s = s.replace("s", "");
                        if let Ok(s) = s.parse::<u64>()
                        {
                            total_seconds += s;
                        }
                    }
                    'h' =>
                    {
                        let s = s.replace("s", "");
                        if let Ok(h) = s.parse::<u64>()
                        {
                            total_seconds += h * 3600;
                        }
                    }
                    _ =>
                    {}
                }
            }
        }
        total_seconds
    }
}
use yansi::Paint;
pub fn format_log_message(level: String, target: String, date: String, message: String) -> String
{
    let level: String = match level.as_str()
    {
        "DEBUG" => Paint::new("debug").bold().to_string(),
        "INFO" => Paint::cyan("information").bold().to_string(),
        "ERROR" => Paint::red("error").bold().to_string(),
        "WARN" => Paint::red("warn").underline().to_string(),
        _ => level,
    };
    let message = Paint::new(message).bold();
    let log = format!(
        "{}@{} on {}: {}",
        level,
        Paint::yellow(target).bold(),
        Paint::green(date).bold(),
        message
    );
    log
}
trait StringUtils
{
    fn captilize(&self) -> Self;
}
impl StringUtils for String
{
    fn captilize(&self) -> Self
    {
        let mut c = self.chars();
        match c.next()
        {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }
}
