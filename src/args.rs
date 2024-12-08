use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Arg {
    #[arg(default_value = "Hello, aiai!")]
    pub content: String,

    /// This will enter chat mode and maintain context across multiple rounds of conversation.
    /// By default, chat messages will be saved in a file named model_date.chat.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub chat: bool,

    /// This will execute ls ${chat_history_file_path}
    #[arg(long = "chat-list", action = clap::ArgAction::SetTrue)]
    pub list: bool,

    /// This will generate a shell script that can be can be executed on time
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub shell: bool,

    /// This can generate code for your content 
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub code: bool,
}
