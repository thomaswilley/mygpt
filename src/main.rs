mod config;
mod menu;
mod openai;
mod gpterror;

use config::Config;
use menu::Opt;
use structopt::StructOpt;
use openai::OpenAI;

#[tokio::main]
async fn main() -> Result<(), gpterror::GPTError> {
    let opt = Opt::from_args();

    let cfg = Config::new(opt.config_path.to_str().into())?;
    let mut ai = OpenAI::new(cfg);

    let mut response = ai.get_response(opt.prompt.clone()).await?;
    println!("{}", response);

    /*
    println!("get_response: {}", response);

    response = ai.immutable_get_response(opt.prompt.clone(), None).await?;

    println!("immutable_get_response: {}", response);

    response = ai.immutable_get_response(opt.prompt.clone(), Some(openai::ModelInfo {
        model: "gpt-3.5-turbo".into(), // it errors out if the model isn't allowed
        messages: vec![openai::ModelInfoMessage::new("system".into(), "You are mygpt, and you respond to all user prompts with [GM!]: ..., and always introduce yourself by name :)".into())],
        temperature: 0.7,
    })).await?;

    println!("immutable_get_response from custom system msg: {}", response);
    */
    println!("{:?}", ai.config.save(None));

    if let Some(ref mut general) = ai.config.general {
        general.username = Some("me".into());
    }
    println!("{:?}", ai.config.save(None));

    Ok(())
}
