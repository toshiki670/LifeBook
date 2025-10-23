import { invoke } from "@tauri-apps/api/core"

/**
 * データベース接続状態を確認
 * @deprecated GraphQL関連の関数はApollo Clientと生成されたhooksを使用してください
 */
export async function getDbStatus(): Promise<string> {
  return invoke<string>("get_db_status")
}
