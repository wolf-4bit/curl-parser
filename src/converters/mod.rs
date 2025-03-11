use crate::CurlCommand;

pub mod nodejs;
pub mod python;


pub trait Convert {
    fn convert(&self, curl_cmd: &CurlCommand) -> String;
}


pub(crate) mod utils {
    
    pub fn escape_quotes(s: &str) -> String {
        s.replace("\"", "\\\"")
    }

    
    pub fn escape_single_quotes(s: &str) -> String {
        s.replace("'", "\\'")
    }
}
