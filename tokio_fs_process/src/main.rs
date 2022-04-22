use std::path::Path;

use anyhow::Context as _;
use::tokio::{fs, process::Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = Path::new("somedir");
    if !dir.exists() {
        fs::create_dir(dir).await?;
    }

    let fut1 = async {
        if !dir.join(".cargo").exists() {
            fs::create_dir(dir.join(".cargo"))
                .await
                .context("Failed")?;
            fs::write(
                dir.join(".cargo").join("config").with_extension("txt"),
                "hello"
            )
            .await
            .context("Failed to create")
        } else {
            anyhow::bail!("error")
        }
    };

    let fut2 = async move {
        // initialize empty git repository if it does not exist
        if !dir.join(".git").exists() {
            let is_succeed = Command::new("git")
                .arg("init")
                .arg(dir)
                .spawn()?
                .wait()
                .await?
                .success();
            if !is_succeed {
                anyhow::bail!("Failed")
            }
            Ok(())
        } else {
            anyhow::bail!("A Git repository is already exists");
        }
    };

    let fut3 = async {
        // create `.gitignore` if it does not exist
        if !dir.join(".gitignore").exists() {
            fs::write(dir.join(".gitignore"), "/target\nCargo.lock\n/.cargo").await.context("Failed")
        } else {
            anyhow::bail!("`.gitignore` is already exists")
        }
    };

    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);
    let handle3 = tokio::spawn(fut3);

    let _ = tokio::join!(handle1, handle2, handle3);

    Ok(())
}
