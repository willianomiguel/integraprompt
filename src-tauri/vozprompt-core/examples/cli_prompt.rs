use std::io::Read;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "cli_claude".into());
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let model = vozprompt_core::cli::model(&id).expect("supported CLI");
    let template = &vozprompt_core::templates::TemplateLibrary::default().templates[0].prompt;
    match vozprompt_core::cli::generate(&id, model, template, &input).await {
        Ok(text) => println!("{text}"),
        Err(code) => {
            eprintln!("{code}");
            std::process::exit(1);
        }
    }
}
