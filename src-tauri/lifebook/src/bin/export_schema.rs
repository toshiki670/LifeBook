use lifebook_lib::graphql_schema::build_schema_without_data;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // スキーマを構築（データベース接続不要）
    let schema = build_schema_without_data();

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
