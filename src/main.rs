use icloud_auth::AppleAccount;
use omnisette::AnisetteConfiguration;
use anyhow::Context;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. 環境変数の読み込み
    dotenv().ok();
    let apple_id = env::var("APPLE_ID")
        .context("環境変数 APPLE_ID が設定されていません")?;
    let apple_pw = env::var("APPLE_APP_PASSWORD")
        .context("環境変数 APPLE_APP_PASSWORD が設定されていません")?;


    // 1. 端末ローカルの Anisette 情報を自動生成（macOS / iOS 互換）
    let anisette_cfg = AnisetteConfiguration::new();
    let mut acc = AppleAccount::new(anisette_cfg).await?;

    // 2. メール & パスワードで SRP ログイン（A2k → M1/M2 まで）
    match acc.login_email_pass(&apple_id, &apple_pw).await? {
        icloud_auth::LoginState::LoggedIn => {
            println!("ログイン完了。GsIdmsToken = {:?}", acc.get_pet());
        }
        next_state => {
            println!("追加認証が必要: {:?}", next_state);
        }
    }

    Ok(())
}