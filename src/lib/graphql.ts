import { invoke } from "@tauri-apps/api/core"

/**
 * データベース接続状態を確認
 */
export async function getDbStatus(): Promise<string> {
  return invoke<string>("get_db_status")
}
