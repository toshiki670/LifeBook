// Settings Feature - Database Settings Form Component

import { useEffect, useState } from "react"
import { Button } from "~/components/ui/button"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { useSettings } from "../hooks/use-settings"

export function DatabaseSettingsForm() {
  const {
    databaseSettings,
    loadingDatabase,
    errorDatabase,
    fetchDatabaseSettings,
    updateDatabaseSettings,
  } = useSettings()

  const [directory, setDirectory] = useState("")

  useEffect(() => {
    fetchDatabaseSettings()
  }, [fetchDatabaseSettings])

  useEffect(() => {
    if (databaseSettings?.databaseDirectory) {
      setDirectory(databaseSettings.databaseDirectory)
    }
  }, [databaseSettings])

  const handleSave = async () => {
    await updateDatabaseSettings(directory)
  }

  const handleSelectDirectory = async () => {
    // TODO: TauriのダイアログAPIを使用してディレクトリ選択を実装
    // import { open } from '@tauri-apps/plugin-dialog'
    // const selected = await open({ directory: true })
    // if (selected) setDirectory(selected)
    console.log("Directory selection will be implemented with Tauri dialog")
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-semibold mb-1">データベース</h2>
        <p className="text-sm text-muted-foreground">
          データベースファイルの保存場所を管理します。
        </p>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="database-directory">データベース保存先</Label>
          <div className="flex gap-2">
            <Input
              id="database-directory"
              value={directory}
              onChange={(e) => setDirectory(e.target.value)}
              placeholder="/path/to/database"
              className="flex-1"
              disabled={loadingDatabase}
            />
            <Button
              type="button"
              variant="outline"
              onClick={handleSelectDirectory}
              disabled={loadingDatabase}
            >
              参照
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            LifeBookのデータベースファイルが保存されるディレクトリです。
          </p>
        </div>

        <Button onClick={handleSave} disabled={loadingDatabase}>
          {loadingDatabase ? "保存中..." : "保存"}
        </Button>

        {errorDatabase && (
          <div className="text-sm text-destructive bg-destructive/10 p-3 rounded-md">
            {errorDatabase}
          </div>
        )}
      </div>
    </div>
  )
}
