# 概要: 設定モーダル導線とカテゴリ別設定画面

## 変更内容

- `AppSidebar` の「設定」ボタンおよびサイドバーアイテムから、`state.backgroundLocation` を付与して `/settings` 配下のルートへ遷移し、元画面を維持したまま設定モーダルを表示。
- `SettingsLayout` を新設し、`Dialog` ベースのモーダル内にカテゴリナビゲーションと `Outlet` を配置。モーダルを閉じた際は背景ルートに戻る制御を追加。
- `routes.ts` に一般/外観/データベースのネストルートを定義し、それぞれの `page.tsx` で `clientLoader`・`clientAction` を実装。Apollo クライアント経由で GraphQL クエリ/ミューテーション (`get*Settings`, `update*Settings`) を発行し、自動保存に対応。
- `components/ui/select.tsx` を追加し、設定フォームの選択 UI を共通化。`Label`・`Button` 等と組み合わせて言語、テーマ、DB パスの入力体験を改善。
- 外観設定では `applyTheme` ユーティリティを呼び出し、選択直後にテーマを適用。App レイアウトの `clientLoader` でも初期テーマを反映し、リスナー破棄を保証。

## フォローアップ

- GraphQL スキーマに追加した設定フィールドの永続化がサーバー側で正しく実装されているか確認する。
- データベース設定の「参照」ボタンから OS のディレクトリ選択ダイアログを呼び出す実装を検討。
- `use-settings` フォルダ内のダミーファイルを整理し、不要であれば削除または実装方針を決める。
- UI テスト／E2E テストでモーダルの開閉とバックナビゲーション（`backgroundLocation`）の挙動を検証。

