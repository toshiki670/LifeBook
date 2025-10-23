use lifebook_lib::{app_state::AppState, database::setup_database, graphql_schema::build_schema};
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // データベース接続を確立
    let db = setup_database().await?;

    // 設定ディレクトリとデータベースディレクトリを一時的なパスに設定
    // スキーマエクスポートには実際のパスは不要
    let config_dir = env::temp_dir().join("lifebook_schema_export_config");
    let default_db_dir = env::temp_dir().join("lifebook_schema_export_db");

    // AppStateを構築
    let app_state = AppState::new(db, config_dir, default_db_dir);

    // スキーマを構築
    let schema = build_schema(app_state);

    // SDL形式でエクスポート
    let sdl = schema.sdl();

    // プロジェクトルートのschema.graphqlファイルに出力
    // src-tauri/ から見て ../ がプロジェクトルート
    let output_path = Path::new("../schema.graphql");

    // ディレクトリが存在しない場合は作成
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // ファイルに書き出し
    fs::write(output_path, &sdl)?;

    println!("Schema exported to schema.graphql");
    println!("Generated {} lines of SDL", sdl.lines().count());

    Ok(())
}
