# Asset コンテキスト - データベース設計

Asset コンテキストのデータベーススキーマ、制約、インデックス戦略を定義します。

---

## テーブル定義

### asset テーブル

```sql
CREATE TABLE asset (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    product_id INT NOT NULL UNIQUE,
    current_quantity DECIMAL(15, 4) NOT NULL DEFAULT 0,
    total_acquisition_cost DECIMAL(15, 2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_asset_product 
        FOREIGN KEY (product_id) 
        REFERENCES product(id) 
        ON DELETE RESTRICT,
    
    CONSTRAINT chk_asset_quantity 
        CHECK (current_quantity >= 0),
    
    CONSTRAINT chk_asset_cost 
        CHECK (total_acquisition_cost >= 0)
);

CREATE INDEX idx_asset_product_id ON asset(product_id);
CREATE INDEX idx_asset_current_quantity ON asset(current_quantity) 
    WHERE current_quantity > 0;  -- 所有中の資産のみ
```

---

### asset_transaction テーブル

```sql
CREATE TABLE asset_transaction (
    id SERIAL PRIMARY KEY,
    type VARCHAR(20) NOT NULL,
    transaction_date DATE NOT NULL,
    quantity DECIMAL(15, 4) NOT NULL,
    unit_price DECIMAL(15, 2),
    unit_cost_at_time DECIMAL(15, 2),
    note TEXT,
    asset_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_transaction_asset 
        FOREIGN KEY (asset_id) 
        REFERENCES asset(id) 
        ON DELETE RESTRICT,
    
    CONSTRAINT chk_transaction_quantity 
        CHECK (quantity > 0),
    
    CONSTRAINT chk_transaction_type 
        CHECK (type IN ('PURCHASE', 'SALE', 'DISPOSAL', 'ADJUSTMENT')),
    
    CONSTRAINT chk_transaction_unit_price 
        CHECK (unit_price IS NULL OR unit_price >= 0),
    
    CONSTRAINT chk_transaction_date 
        CHECK (transaction_date <= CURRENT_DATE)
);

CREATE INDEX idx_transaction_asset_id ON asset_transaction(asset_id);
CREATE INDEX idx_transaction_date ON asset_transaction(transaction_date DESC);
CREATE INDEX idx_transaction_type ON asset_transaction(type);
```

**注意**: `updated_at` は不要（履歴は変更しないため）

---

### product テーブル

```sql
CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    unit VARCHAR(50) NOT NULL,
    manufacturer_id INT NOT NULL,
    deleted_at DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_product_manufacturer 
        FOREIGN KEY (manufacturer_id) 
        REFERENCES manufacturer(id) 
        ON DELETE RESTRICT,
    
    CONSTRAINT chk_product_unit 
        CHECK (unit IN ('PIECE', 'BOX', 'PACK', 'WEIGHT', 'VOLUME'))
);

CREATE INDEX idx_product_manufacturer_id ON product(manufacturer_id);
CREATE INDEX idx_product_deleted_at ON product(deleted_at);
CREATE INDEX idx_product_active ON product(deleted_at) WHERE deleted_at IS NULL;
```

---

### category テーブル

```sql
CREATE TABLE category (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    parent_id INT,
    depth INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_category_parent 
        FOREIGN KEY (parent_id) 
        REFERENCES category(id) 
        ON DELETE RESTRICT,
    
    CONSTRAINT chk_category_depth 
        CHECK (depth >= 0),
    
    CONSTRAINT chk_category_depth_limit 
        CHECK (depth <= 10)  -- 最大10階層（推奨は5階層）
);

CREATE INDEX idx_category_parent_id ON category(parent_id);
CREATE INDEX idx_category_depth ON category(depth);
CREATE INDEX idx_category_root ON category(parent_id) WHERE parent_id IS NULL;
```

---

### product_category テーブル

```sql
CREATE TABLE product_category (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    category_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_product_category_product 
        FOREIGN KEY (product_id) 
        REFERENCES product(id) 
        ON DELETE CASCADE,
    
    CONSTRAINT fk_product_category_category 
        FOREIGN KEY (category_id) 
        REFERENCES category(id) 
        ON DELETE RESTRICT,
    
    CONSTRAINT uq_product_category 
        UNIQUE (product_id, category_id)
);

CREATE INDEX idx_product_category_product_id ON product_category(product_id);
CREATE INDEX idx_product_category_category_id ON product_category(category_id);
```

---

### manufacturer テーブル

```sql
CREATE TABLE manufacturer (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    website VARCHAR(500),
    contact_email VARCHAR(255),
    deleted_at DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT uq_manufacturer_name 
        UNIQUE (name),
    
    CONSTRAINT chk_manufacturer_email 
        CHECK (contact_email IS NULL OR contact_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$')
);

CREATE INDEX idx_manufacturer_deleted_at ON manufacturer(deleted_at);
CREATE INDEX idx_manufacturer_active ON manufacturer(deleted_at) WHERE deleted_at IS NULL;
```

---

## インデックス戦略

### パフォーマンス最適化

#### 1. 所有中の資産を高速取得

```sql
CREATE INDEX idx_asset_current_quantity ON asset(current_quantity) 
    WHERE current_quantity > 0;
```

**用途**: 所有中の資産一覧を高速表示

---

#### 2. アクティブなマスタのみ高速取得

```sql
CREATE INDEX idx_product_active ON product(deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX idx_manufacturer_active ON manufacturer(deleted_at) WHERE deleted_at IS NULL;
```

**用途**: 新規登録時の選択肢を高速表示（削除済みを除外）

---

#### 3. ルートカテゴリの高速取得

```sql
CREATE INDEX idx_category_root ON category(parent_id) WHERE parent_id IS NULL;
```

**用途**: カテゴリツリーのルート一覧を高速表示

---

#### 4. トランザクション履歴の時系列取得

```sql
CREATE INDEX idx_transaction_date ON asset_transaction(transaction_date DESC);
```

**用途**: 最新のトランザクションから順に表示

---

#### 5. カテゴリの階層クエリ最適化

```sql
CREATE INDEX idx_category_parent_id ON category(parent_id);
CREATE INDEX idx_category_depth ON category(depth);
```

**用途**: 
- 子カテゴリの取得
- 特定階層のカテゴリ取得

---

## 制約一覧

### 主キー制約

すべてのテーブルに `id` を主キーとして設定。

---

### 外部キー制約

| テーブル | カラム | 参照先 | ON DELETE |
|---------|--------|--------|-----------|
| asset | product_id | product(id) | RESTRICT |
| asset_transaction | asset_id | asset(id) | RESTRICT |
| product | manufacturer_id | manufacturer(id) | RESTRICT |
| product_category | product_id | product(id) | CASCADE |
| product_category | category_id | category(id) | RESTRICT |
| category | parent_id | category(id) | RESTRICT |

**RESTRICT の理由**:
- 削除時のビジネスロジック（削除拒否、孤児昇格等）をアプリケーション層で制御

**CASCADE の理由**:
- product_category は Product のライフサイクルに従う

---

### ユニーク制約

| テーブル | カラム | 説明 |
|---------|--------|------|
| asset | product_id | 1つの商品に1つの資産のみ（1対1） |
| product_category | (product_id, category_id) | 同じ商品-カテゴリの組み合わせは1回のみ |
| manufacturer | name | メーカー名は一意 |

---

### チェック制約

#### asset

```sql
CHECK (current_quantity >= 0)
CHECK (total_acquisition_cost >= 0)
```

**理由**: 数量と原価は負数になり得ない

---

#### asset_transaction

```sql
CHECK (quantity > 0)
CHECK (type IN ('PURCHASE', 'SALE', 'DISPOSAL', 'ADJUSTMENT'))
CHECK (unit_price IS NULL OR unit_price >= 0)
CHECK (transaction_date <= CURRENT_DATE)
```

**理由**:
- 数量は正数のみ（減少は SALE/DISPOSAL で表現）
- 未来日のトランザクションは不可

---

#### category

```sql
CHECK (depth >= 0)
CHECK (depth <= 10)  -- 推奨は5階層
```

**理由**: 階層が深すぎるとパフォーマンスとUXが悪化

---

#### manufacturer

```sql
CHECK (contact_email IS NULL OR contact_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$')
```

**理由**: メールアドレスの形式チェック

---

## バリデーションルール

アプリケーション層で実施するバリデーションルールです。

### Asset

| 項目 | ルール | エラーメッセージ |
|-----|--------|----------------|
| name | 必須、空文字列不可 | "Asset name cannot be empty" |
| name | 最大200文字 | "Asset name must be 200 characters or less" |
| current_quantity | 0以上 | "Quantity must be non-negative" |
| total_acquisition_cost | 0以上 | "Cost must be non-negative" |

---

### AssetTransaction

| 項目 | ルール | エラーメッセージ |
|-----|--------|----------------|
| quantity | 0より大きい | "Quantity must be greater than zero" |
| unit_price | PURCHASE/SALE時は必須 | "Unit price is required for purchase/sale" |
| unit_price | 0以上 | "Unit price must be non-negative" |
| transaction_date | 未来日不可 | "Transaction date cannot be in the future" |
| quantity | 売却/廃棄時は所有数以下 | "Insufficient quantity" |

---

### Product

| 項目 | ルール | エラーメッセージ |
|-----|--------|----------------|
| name | 必須、空文字列不可 | "Product name cannot be empty" |
| name | 最大200文字 | "Product name must be 200 characters or less" |
| manufacturer_id | 存在するメーカー | "Manufacturer not found" |
| manufacturer_id | アクティブなメーカー（新規登録時） | "Manufacturer is not active" |

---

### Category

| 項目 | ルール | エラーメッセージ |
|-----|--------|----------------|
| name | 必須、空文字列不可 | "Category name cannot be empty" |
| name | 最大100文字 | "Category name must be 100 characters or less" |
| parent_id | 存在するカテゴリ | "Parent category not found" |
| parent_id | 自分自身不可 | "Cannot set self as parent" |
| parent_id | 自分の子孫不可 | "Cannot create cycle" |
| depth | 推奨最大5階層 | "Maximum recommended depth of 5 exceeded" |

---

### Manufacturer

| 項目 | ルール | エラーメッセージ |
|-----|--------|----------------|
| name | 必須、空文字列不可 | "Manufacturer name cannot be empty" |
| name | 最大200文字 | "Manufacturer name must be 200 characters or less" |
| name | ユニーク | "Manufacturer name already exists" |
| contact_email | メールアドレス形式 | "Invalid email format" |

---

## データ型の精度

### Decimal 型の定義

| カラム | 精度 | 理由 |
|--------|------|------|
| `current_quantity` | DECIMAL(15, 4) | 小数第4位まで対応（0.0001個単位） |
| `total_acquisition_cost` | DECIMAL(15, 2) | 通常の金額（小数第2位＝銭単位） |
| `unit_price` | DECIMAL(15, 2) | 通常の金額 |
| `unit_cost_at_time` | DECIMAL(15, 2) | 通常の金額 |

**最大値**: 
- DECIMAL(15, 4): 9,999,999,999.9999
- DECIMAL(15, 2): 999,999,999,999.99

---

## マイグレーション順序

テーブル作成は以下の順序で実行する必要があります（外部キー依存）。

```
1. manufacturer
2. category
3. product
4. product_category
5. asset
6. asset_transaction
```

---

## 削除制約のまとめ

| 削除対象 | 依存関係 | 制約 | 挙動 |
|---------|---------|------|------|
| manufacturer | product が参照 | RESTRICT | Product紐付き時は削除不可 |
| category | product_category が参照 | RESTRICT | Product紐付き時は削除不可 |
| category | category(parent) が参照 | RESTRICT | 子カテゴリがある場合は削除不可（孤児昇格後に削除） |
| product | product_category が参照 | - | ProductCategoryは自動削除される |
| product | asset が参照 | RESTRICT | Asset紐付き時は削除不可 |
| asset | asset_transaction が参照 | RESTRICT | 削除機能なし |

---

## クエリパターン

### よく使われるクエリとインデックス

#### 1. 所有中の資産一覧

```sql
SELECT * FROM asset 
WHERE current_quantity > 0
ORDER BY updated_at DESC;
```

**使用インデックス**: `idx_asset_current_quantity`

---

#### 2. 資産の詳細とトランザクション履歴

```sql
SELECT 
    a.*,
    at.type,
    at.transaction_date,
    at.quantity,
    at.unit_price,
    at.note
FROM asset a
LEFT JOIN asset_transaction at ON a.id = at.asset_id
WHERE a.id = ?
ORDER BY at.transaction_date DESC;
```

**使用インデックス**: `idx_transaction_asset_id`, `idx_transaction_date`

---

#### 3. アクティブな商品一覧

```sql
SELECT * FROM product 
WHERE deleted_at IS NULL
ORDER BY name;
```

**使用インデックス**: `idx_product_active`

---

#### 4. カテゴリツリーの取得

```sql
-- ルートカテゴリ
SELECT * FROM category 
WHERE parent_id IS NULL
ORDER BY name;

-- 特定カテゴリの子
SELECT * FROM category 
WHERE parent_id = ?
ORDER BY name;
```

**使用インデックス**: `idx_category_root`, `idx_category_parent_id`

---

#### 5. カテゴリの全子孫を取得

```sql
WITH RECURSIVE descendants AS (
    SELECT id, name, parent_id, depth
    FROM category
    WHERE id = ?
    
    UNION ALL
    
    SELECT c.id, c.name, c.parent_id, c.depth
    FROM category c
    JOIN descendants d ON c.parent_id = d.id
)
SELECT * FROM descendants
WHERE id != ?
ORDER BY depth, name;
```

---

#### 6. 特定商品のカテゴリ一覧

```sql
SELECT c.*
FROM category c
JOIN product_category pc ON c.id = pc.category_id
WHERE pc.product_id = ?
ORDER BY c.name;
```

**使用インデックス**: `idx_product_category_product_id`

---

#### 7. 特定カテゴリの商品一覧

```sql
SELECT p.*
FROM product p
JOIN product_category pc ON p.id = pc.product_id
WHERE pc.category_id = ?
  AND p.deleted_at IS NULL
ORDER BY p.name;
```

**使用インデックス**: `idx_product_category_category_id`, `idx_product_active`

---

## パフォーマンス考慮事項

### 1. キャッシュフィールドの活用

- `Asset.current_quantity`: トランザクションを集計せずに所有数を即座に取得
- `Asset.total_acquisition_cost`: トランザクションを集計せずに原価を即座に取得
- `Category.depth`: 親を再帰的に辿らずに階層深度を即座に取得

**トレードオフ**: 更新時にキャッシュを同期する必要がある

---

### 2. 部分インデックス

```sql
-- 所有中の資産のみインデックス
CREATE INDEX idx_asset_current_quantity ON asset(current_quantity) 
    WHERE current_quantity > 0;

-- アクティブな商品のみインデックス
CREATE INDEX idx_product_active ON product(deleted_at) 
    WHERE deleted_at IS NULL;
```

**メリット**: インデックスサイズが小さくなり、更新コストも削減

---

### 3. 複合インデックス（将来検討）

頻繁に組み合わせて検索する場合に追加を検討：

```sql
-- 資産一覧をカテゴリで絞り込む場合
CREATE INDEX idx_product_category_category_product 
    ON product_category(category_id, product_id);

-- トランザクション履歴を種別で絞り込む場合
CREATE INDEX idx_transaction_asset_type 
    ON asset_transaction(asset_id, type);
```

---

## データ整合性の保証

### トランザクション境界

以下の操作は同一データベーストランザクション内で実行する必要があります。

#### AssetTransaction 追加時

```rust
async fn add_transaction(/* ... */) -> Result<()> {
    let mut tx = db.begin().await?;
    
    // 1. Asset のキャッシュを更新
    asset.update_quantity_and_cost(/* ... */);
    asset.save(&mut tx).await?;
    
    // 2. AssetTransaction を追加
    transaction.insert(&mut tx).await?;
    
    tx.commit().await?;
    Ok(())
}
```

---

#### Category 削除時（孤児昇格）

```rust
async fn delete_category(/* ... */) -> Result<()> {
    let mut tx = db.begin().await?;
    
    // 1. 子カテゴリの親とdepthを更新
    for child in children {
        child.update(&mut tx).await?;
    }
    
    // 2. カテゴリを削除
    category.delete(&mut tx).await?;
    
    tx.commit().await?;
    Ok(())
}
```

---

## 初期データ

システム初期化時に登録する推奨マスタデータです。

### デフォルトカテゴリ

```sql
INSERT INTO category (name, parent_id, depth) VALUES
    ('家電', NULL, 0),
    ('衣類', NULL, 0),
    ('書籍', NULL, 0),
    ('家具', NULL, 0),
    ('その他', NULL, 0);
```

---

### デフォルトメーカー

```sql
INSERT INTO manufacturer (name) VALUES
    ('不明'),
    ('ノーブランド');
```

**用途**: メーカー情報が不明な商品の登録時に使用

---

## 参照

- [models.md](./models.md): モデル定義と関係性
- [business-rules.md](./business-rules.md): ビジネスルールと削除ポリシー
- [ubiquitous-language.md](./ubiquitous-language.md): ドメイン用語の定義

