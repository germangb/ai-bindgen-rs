use ai_bindgen::ai;

#[ai]
extern "C" {
    /// The parameters more-or-less match those from the OpenAI API.
    /// Refer to docs in https://platform.openai.com/docs/api-reference/chat/create/
    #[ai(
        prompt = r#"Simply prints hello world to stdout, but in punjabi"#,
        model = "gpt-4.1-nano",
        temperature = 1.0,
        presence_penalty = 0.0,
        frequency_penalty = 0.0,
        top_p = 1.0,
        max_tokens = 100
    )]
    fn hello_world();
}

fn main() {
    hello_world(); // ਹਲੋ ਵਰਲਡ
}
