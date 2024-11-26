# aiai(哎哎)
A command-line productivity tool powered by AI large language models like llama3.2

> Original idea: https://github.com/TheR1D/shell_gpt
> 
> This program features: `Chat Mode`, `REPL Mode`, `Shell commands`, `Shell integration`, `Generating code`, `Function calling`...

I want more features like ...~~(currently no idea)~~ Honestly, I just want to use Rust to build this tool.

## Clean Mind
OpenAI API => Ollama, ChatGPT API

Runtime configuration file:
1. api_url
2. api_key
3. default_chat_model
4. disable_streaming
5. chat_cache_path
6. chat_cache_max_length
7. request_timeout
8. default_execute_shell_cmd
9. markdown_theme_view

Arguments:

Required: 

content: single chat content
> Example: `aiai "why is the sky blue?"`
> 
> More complex: `aiai "what is my most used command" < ~/.bash_history`
>
> `tail -n 30 log_info.log | aiai "help me to find error"`

Optional:

--chat
> Example: `aiai --chat "let's start to chat"`
>
> This will enter chat mode and maintain context across multiple rounds of conversation. By default, chat messages will be saved in a file named `model_date.chat`.

--chat-list
> Example: `aiai --chat-list`
>
> This will execute `ls ${chat_history_file_path}`

--shell -s
> Example: `aiai --shell "update my system"`

--code -c
> Example: `aiai --code "String(2024-01-08 00:00) => date in java"`

## Step by Step
- [ ] Determine how to use prompt to improve model stdout
- [ ] Find a way to use custom prompt in chat mode
- [ ] Implement chat mode
- [ ] Implement history chat mode
- [ ] Generate and execute shell commands
- [ ] Integrate shell commands
- [ ] Upload files (txt, images) to model and generate statistics
- [ ] Help to write code or shell scripts

## Need to Learn
1. CLI program => clap
2. Async programming => tokio
3. Connect to model => reqwest
4. Markdown view