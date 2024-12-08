use aiai::{args, just_content, println_response};
use clap::Parser;

fn main() {
    let args = args::Arg::parse();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let response = just_content(args.content).await.unwrap();
        println_response(response).await;
    })
}
