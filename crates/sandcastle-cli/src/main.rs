use std::path::PathBuf;

use clap::Parser;
use sandcastle_core::domain::repositories::models::GitOpsPlatformType;

#[derive(Debug, clap::Parser)]
#[command(version)]
#[command(about)]
pub enum SandcastleCli {
    Serve,
    Test {
        #[arg(short = 'f', long = "file")]
        file: PathBuf,
        #[arg(short = 'g', long = "gitops-platform")]
        gitops_platform: GitOpsPlatformType,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = SandcastleCli::parse();
    match cli {
        SandcastleCli::Serve => sandcastle_core::application::start().await,
        SandcastleCli::Test {
            file,
            gitops_platform,
        } => {
            let applications =
                sandcastle_core::application::test_application(file, gitops_platform).await?;
            for application in applications.split("---") {
                println!("{}", application);
            }
            Ok(())
        }
    }
}
