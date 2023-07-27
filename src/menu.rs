use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "mygpt",
    about = "your helpful openai gpt-4-based assistant, using your own API keys.",
)]
pub struct Opt {
    #[structopt(name = "prompt")]
    pub prompt: String,

    #[structopt(short = "c", long = "config", help = "full path to config file (e.g., mygpt.conf)", validator = is_file, default_value = "mygpt.conf")]
    pub config_path: PathBuf,

    #[structopt(short = "k", 
                long = "openai_api_key",
                help = "best practice is to put your key in mygpt.conf file and pass that in. but if you must, you can pass it in directly.",
               )]
    pub openai_api_key: Option<String>,
}

fn is_file(path: String) -> Result<(), String> {
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_file() {
                Ok(())
            } else {
                Err(format!("{} is not a file", path))
            }
        },
        Err(_) => Err(format!("Could not read conf file at {}", path)),
    }
}
