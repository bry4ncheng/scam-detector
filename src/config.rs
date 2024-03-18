use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(env, long)]
    pub hf_token : String,

    #[clap(env, long)]
    pub origin_urls: String
}
