use clap::Parser;

#[derive(Debug, clap::Parser)]
pub enum SandcastleCli {
    Serve,
}

#[tokio::main]
async fn main() {
    let cli = SandcastleCli::parse();
    match cli {
        SandcastleCli::Serve => {
            sandcastle_core::start().await
        }
    }
}
