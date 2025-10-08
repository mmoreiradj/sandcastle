use clap::Parser;

#[derive(Debug, clap::Parser)]
pub enum SandcastleCli {
    Serve,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = SandcastleCli::parse();
    match cli {
        SandcastleCli::Serve => {
            sandcastle_core::application::start().await
        }
    }
}
