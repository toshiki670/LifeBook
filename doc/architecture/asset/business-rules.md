# Asset コンテキスト - ビジネスルール

Asset コンテキストの主要なビジネスルールとドメインロジックを定義します。

---

## 削除ポリシー

各エンティティの削除方式と条件を定義します。

| エンティティ | 削除方式 | 条件 |
|------------|---------|------|
| **Asset** | 削除不可 | 削除機能なし（履歴として永続保存） |
| **AssetTransaction** | 削除不可 | 履歴は永続保存 |
| **Product** | ソフトデリート（`deleted_at`） | Asset 紐付き時は削除拒否 |
| **Category** | 孤児昇格 + ハードデリート | Product 紐付き時は削除拒否、子カテゴリは親に昇格 |
| **Manufacturer** | ソフトデリート（`deleted_at`） | Product 紐付き時は削除拒否 |
| **ProductCategory** | カスケード削除 | Product 削除時に自動削除 |

---

## Category 削除の詳細

### 孤児昇格（Orphan Promotion）

カテゴリを削除する際、子カテゴリを親カテゴリに繋ぎ直す操作です。

#### ロジック

```rust
pub async fn delete_category(category: &Category) -> Result<()> {
    // 1. Product紐付きチェック
    if category.has_products().await? {
        return Err("Cannot delete: category has products");
    }

    // 2. 子カテゴリを親に昇格（depth - 1）
    for child in category.get_children().await? {
        child.parent_id = category.parent_id;
        child.update_subtree_depth(-1).await?;
        child.save().await?;
    }

    // 3. 完全削除（ハードデリート）
    category.hard_delete().await?;
}
```

#### 例

**削除前**:
```
家電 (id: 1, depth: 0)
└─ オーディオ機器 (id: 2, depth: 1) ← 削除対象
   ├─ イヤホン (id: 3, depth: 2)
   └─ スピーカー (id: 4, depth: 2)
```

**削除後**:
```
家電 (id: 1, depth: 0)
├─ イヤホン (id: 3, depth: 1) ← 昇格
└─ スピーカー (id: 4, depth: 1) ← 昇格
```

---

## 循環参照の防止

カテゴリの親変更時に、自分の子孫を親に設定できないようにするバリデーションです。

### ロジック

```rust
pub async fn change_parent(category: &mut Category, new_parent_id: Option<i32>) -> Result<()> {
    if let Some(pid) = new_parent_id {
        // 自分自身を親にできない
        if pid == category.id {
            return Err("Cannot set self as parent");
        }

        // 自分の子孫を親にできない
        if category.is_ancestor_of(pid).await? {
            return Err("Cannot create cycle");
        }
    }

    // depth更新処理...
}

async fn is_ancestor_of(&self, category_id: i32) -> Result<bool> {
    let mut current_id = category_id;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&current_id) {
            return Ok(true);  // ループ検出
        }
        visited.insert(current_id);

        let category = Category::find_by_id(current_id).await?;

        match category.parent_id {
            None => return Ok(false),  // ルートに到達
            Some(parent_id) => {
                if parent_id == self.id {
                    return Ok(true);  // 自分に到達（子孫である）
                }
                current_id = parent_id;
            }
        }
    }
}
```

### 例

```
家電 (id: 1)
└─ オーディオ機器 (id: 2)
   └─ イヤホン (id: 3)

NG: 「家電」の親を「イヤホン」に変更
  → 家電 → オーディオ機器 → イヤホン → 家電 → ... (循環)
  → エラー: "Cannot create cycle"
```

---

## 移動平均法による原価計算

購入の都度、平均取得単価を再計算する原価計算方法です。

### 基本ロジック

```rust
impl Asset {
    pub fn add_purchase(&mut self, quantity: Decimal, unit_price: Decimal) {
        self.current_quantity += quantity;
        self.total_acquisition_cost += quantity * unit_price;
    }

    pub fn add_sale(&mut self, quantity: Decimal) -> Result<Decimal> {
        if quantity > self.current_quantity {
            return Err(DomainError::InsufficientQuantity);
        }

        // 平均取得単価で按分
        let avg_unit_cost = self.total_acquisition_cost / self.current_quantity;
        let cost_to_remove = quantity * avg_unit_cost;

        self.current_quantity -= quantity;
        self.total_acquisition_cost -= cost_to_remove;

        Ok(avg_unit_cost)  // 売却損益計算用に返す
    }

    pub fn average_unit_cost(&self) -> Decimal {
        if self.current_quantity == Decimal::ZERO {
            Decimal::ZERO
        } else {
            self.total_acquisition_cost / self.current_quantity
        }
    }
}
```

---

### 計算例

#### 初期状態

```
current_quantity: 0
total_acquisition_cost: 0
```

---

#### 1. 購入: 2個 @ 5,000円

```
新しい数量: 0 + 2 = 2個
新しい総取得原価: 0 + 10,000 = 10,000円
平均取得単価: 10,000 ÷ 2 = 5,000円/個

→ Asset更新
  current_quantity: 2
  total_acquisition_cost: 10,000

→ AssetTransaction記録
  type: PURCHASE
  quantity: 2
  unit_price: 5,000
  unit_cost_at_time: 5,000
```

---

#### 2. 購入: 3個 @ 4,000円

```
新しい数量: 2 + 3 = 5個
新しい総取得原価: 10,000 + 12,000 = 22,000円
平均取得単価: 22,000 ÷ 5 = 4,400円/個

→ Asset更新
  current_quantity: 5
  total_acquisition_cost: 22,000

→ AssetTransaction記録
  type: PURCHASE
  quantity: 3
  unit_price: 4,000
  unit_cost_at_time: 4,400
```

**ポイント**: 平均単価が5,000円から4,400円に下がった

---

#### 3. 売却: 1個 @ 3,500円

```
その時点の平均取得単価: 22,000 ÷ 5 = 4,400円/個
減らす原価: 4,400 × 1 = 4,400円

新しい数量: 5 - 1 = 4個
新しい総取得原価: 22,000 - 4,400 = 17,600円
平均取得単価: 17,600 ÷ 4 = 4,400円/個（変わらず）

売却損益: 3,500 - 4,400 = -900円（損失）

→ Asset更新
  current_quantity: 4
  total_acquisition_cost: 17,600

→ AssetTransaction記録
  type: SALE
  quantity: 1
  unit_price: 3,500（売却価格）
  unit_cost_at_time: 4,400（その時点の平均取得単価）
  note: "売却損益: -900円"
```

**ポイント**: 
- 売却しても平均単価は変わらない
- `unit_cost_at_time` に記録することで、後から損益を計算可能

---

### 廃棄の場合

```rust
impl Asset {
    pub fn add_disposal(&mut self, quantity: Decimal) -> Result<Decimal> {
        // 売却と同じロジック（価格はゼロ）
        self.add_sale(quantity)
    }
}
```

廃棄も売却と同様に平均取得単価で原価を減算します。

```
→ AssetTransaction記録
  type: DISPOSAL
  quantity: 1
  unit_price: null（廃棄なので売却価格なし）
  unit_cost_at_time: 4,400
  note: "故障により廃棄"
```

---

## カテゴリ階層の操作

### 親子間挿入（Insert Between）

既存の親子関係の間に新しいカテゴリを挿入する操作です。

#### ロジック

```rust
pub async fn create_between(
    name: String,
    parent_id: i32,
    children_to_adopt: Vec<i32>
) -> Result<Category> {
    let parent = Category::find_by_id(parent_id).await?;

    // 1. 新しいカテゴリを作成
    let mut new_category = Category {
        id: 0,
        name,
        parent_id: Some(parent_id),
        depth: parent.depth + 1,
    };

    new_category = new_category.insert().await?;

    // 2. 指定された子を養子にする
    for child_id in children_to_adopt {
        let mut child = Category::find_by_id(child_id).await?;

        // 安全性チェック
        if child.parent_id != Some(parent_id) {
            new_category.delete().await?;
            return Err(DomainError::ValidationError(
                format!("Child {} is not a direct child of parent {}", child_id, parent_id)
            ));
        }

        // 親を新カテゴリに変更
        child.parent_id = Some(new_category.id);

        // 子孫のdepthを+1
        child.update_subtree_depth(1).await?;
        child.save().await?;
    }

    Ok(new_category)
}
```

#### 例

**挿入前**:
```
家電 (id: 1, depth: 0)
├─ イヤホン (id: 3, depth: 1)
└─ スピーカー (id: 4, depth: 1)
```

**操作**: 「オーディオ機器」を家電の下に作成し、イヤホンとスピーカーを配下に

```rust
Category::create_between(
    "オーディオ機器".to_string(),
    parent_id: 1,  // 家電
    children_to_adopt: vec![3, 4]  // イヤホン、スピーカー
).await?;
```

**挿入後**:
```
家電 (id: 1, depth: 0)
└─ オーディオ機器 (id: 2, depth: 1) ← 新規挿入
   ├─ イヤホン (id: 3, depth: 2) ← depth+1
   └─ スピーカー (id: 4, depth: 2) ← depth+1
```

---

## キャッシュ更新ルール

### Asset のキャッシュ

`current_quantity` と `total_acquisition_cost` は、トランザクション追加時に自動更新されます。

```rust
pub async fn add_transaction(
    asset: &mut Asset,
    transaction_type: TransactionType,
    quantity: Decimal,
    unit_price: Option<Decimal>
) -> Result<AssetTransaction> {
    let unit_cost_at_time = match transaction_type {
        TransactionType::Purchase => {
            // 購入: キャッシュを更新
            asset.add_purchase(quantity, unit_price.unwrap());
            asset.average_unit_cost()
        }
        TransactionType::Sale | TransactionType::Disposal => {
            // 売却/廃棄: その時点の平均単価を記録
            let avg_cost = asset.average_unit_cost();
            asset.add_sale(quantity)?;
            avg_cost
        }
        TransactionType::Adjustment => {
            // 調整: 手動で current_quantity を設定
            asset.average_unit_cost()
        }
    };

    // トランザクション記録
    let transaction = AssetTransaction {
        type: transaction_type,
        transaction_date: Utc::now().date_naive(),
        quantity,
        unit_price,
        unit_cost_at_time: Some(unit_cost_at_time),
        note: None,
        asset_id: asset.id,
    };

    transaction.insert().await
}
```

**重要**: Asset の更新とトランザクションの追加は同一トランザクション内で実行する必要があります。

---

### Category の depth キャッシュ

`depth` は以下のタイミングで更新されます。

#### 1. 作成時

```rust
pub async fn create(name: String, parent_id: Option<i32>) -> Result<Category> {
    let depth = if let Some(pid) = parent_id {
        let parent = Category::find_by_id(pid).await?;
        parent.depth + 1
    } else {
        0  // ルートカテゴリ
    };

    Category::insert(name, parent_id, depth).await
}
```

---

#### 2. 親変更時

```rust
pub async fn change_parent(&mut self, new_parent_id: Option<i32>) -> Result<()> {
    // 循環参照チェック（省略）

    let new_depth = if let Some(pid) = new_parent_id {
        let parent = Category::find_by_id(pid).await?;
        parent.depth + 1
    } else {
        0
    };

    let depth_diff = new_depth - self.depth;

    // 自分と子孫すべてのdepthを更新
    self.update_subtree_depth(depth_diff).await?;
}

async fn update_subtree_depth(&mut self, depth_diff: i32) -> Result<()> {
    sqlx::query!(
        "WITH RECURSIVE descendants AS (
            SELECT id FROM category WHERE id = ?
            UNION ALL
            SELECT c.id FROM category c
            JOIN descendants d ON c.parent_id = d.id
        )
        UPDATE category 
        SET depth = depth + ?
        WHERE id IN (SELECT id FROM descendants)",
        self.id, depth_diff
    ).execute(&db).await?;

    Ok(())
}
```

---

#### 3. 削除時（孤児昇格）

```rust
// 子カテゴリのdepthを-1
for child in category.get_children().await? {
    child.depth -= 1;
    child.update_subtree_depth(-1).await?;  // 子孫も-1
}
```

---

## バリデーションルール

### Asset

| フィールド | ルール |
|-----------|--------|
| `name` | 必須、空文字列不可、最大200文字 |
| `current_quantity` | 0以上 |
| `total_acquisition_cost` | 0以上 |

```rust
pub fn new(name: String, product_id: i32) -> Result<Self, DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::ValidationError("Asset name cannot be empty".into()));
    }
    
    if name.len() > 200 {
        return Err(DomainError::ValidationError("Asset name must be 200 characters or less".into()));
    }

    Ok(Self {
        id: None,
        name: name.trim().to_string(),
        description: None,
        product_id,
        current_quantity: Decimal::ZERO,
        total_acquisition_cost: Decimal::ZERO,
    })
}
```

---

### AssetTransaction

| フィールド | ルール |
|-----------|--------|
| `quantity` | 0より大きい |
| `unit_price` | PURCHASE/SALE時は必須、0以上 |
| `transaction_date` | 未来日不可 |

```rust
pub fn new(
    type_: TransactionType,
    quantity: Decimal,
    unit_price: Option<Decimal>,
    transaction_date: NaiveDate,
) -> Result<Self, DomainError> {
    if quantity <= Decimal::ZERO {
        return Err(DomainError::ValidationError("Quantity must be greater than zero".into()));
    }

    if matches!(type_, TransactionType::Purchase | TransactionType::Sale) {
        if unit_price.is_none() {
            return Err(DomainError::ValidationError("Unit price is required for purchase/sale".into()));
        }
        if unit_price.unwrap() < Decimal::ZERO {
            return Err(DomainError::ValidationError("Unit price must be non-negative".into()));
        }
    }

    if transaction_date > Utc::now().date_naive() {
        return Err(DomainError::ValidationError("Transaction date cannot be in the future".into()));
    }

    Ok(Self { /* ... */ })
}
```

---

### Product

| フィールド | ルール |
|-----------|--------|
| `name` | 必須、空文字列不可、最大200文字 |
| `manufacturer_id` | 存在するメーカーを参照 |

```rust
pub fn new(
    name: String,
    manufacturer_id: i32,
    unit: MeasurementUnit,
) -> Result<Self, DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::ValidationError("Product name cannot be empty".into()));
    }

    if name.len() > 200 {
        return Err(DomainError::ValidationError("Product name must be 200 characters or less".into()));
    }

    Ok(Self {
        id: None,
        name: name.trim().to_string(),
        description: None,
        unit,
        manufacturer_id,
        deleted_at: None,
    })
}
```

---

### Category

| フィールド | ルール |
|-----------|--------|
| `name` | 必須、空文字列不可、最大100文字 |
| `parent_id` | 存在するカテゴリを参照、循環参照禁止 |
| `depth` | 推奨最大5階層 |

```rust
pub fn new(name: String, parent_id: Option<i32>) -> Result<Self, DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::ValidationError("Category name cannot be empty".into()));
    }

    if name.len() > 100 {
        return Err(DomainError::ValidationError("Category name must be 100 characters or less".into()));
    }

    Ok(Self {
        id: None,
        name: name.trim().to_string(),
        parent_id,
        depth: 0,  // 作成時に計算
    })
}

pub async fn validate_max_depth(&self, max_depth: i32) -> Result<(), DomainError> {
    if self.depth >= max_depth {
        return Err(DomainError::ValidationError(
            format!("Maximum depth of {} exceeded", max_depth)
        ));
    }
    Ok(())
}
```

---

### Manufacturer

| フィールド | ルール |
|-----------|--------|
| `name` | 必須、空文字列不可、最大200文字、ユニーク |
| `contact_email` | メールアドレス形式（オプション） |

```rust
pub fn new(name: String) -> Result<Self, DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::ValidationError("Manufacturer name cannot be empty".into()));
    }

    if name.len() > 200 {
        return Err(DomainError::ValidationError("Manufacturer name must be 200 characters or less".into()));
    }

    Ok(Self {
        id: None,
        name: name.trim().to_string(),
        website: None,
        contact_email: None,
        deleted_at: None,
    })
}

pub fn set_contact_email(&mut self, email: String) -> Result<(), DomainError> {
    if !email.trim().is_empty() && !is_valid_email(&email) {
        return Err(DomainError::ValidationError("Invalid email format".into()));
    }
    
    self.contact_email = if email.trim().is_empty() {
        None
    } else {
        Some(email.trim().to_string())
    };
    
    Ok(())
}
```

---

## ドメインイベント（将来拡張）

以下のドメインイベントを発行することで、監査ログや通知機能を実装できます。

```rust
// 購入時
AssetPurchased {
    asset_id: i32,
    quantity: Decimal,
    unit_price: Decimal,
    transaction_date: NaiveDate,
}

// 売却時
AssetSold {
    asset_id: i32,
    quantity: Decimal,
    unit_price: Decimal,
    profit_loss: Decimal,  // 売却損益
    transaction_date: NaiveDate,
}

// 廃棄時
AssetDisposed {
    asset_id: i32,
    quantity: Decimal,
    loss: Decimal,  // 損失額
    transaction_date: NaiveDate,
}

// カテゴリ削除時
CategoryDeleted {
    category_id: i32,
    orphan_promoted: Vec<i32>,  // 昇格した子カテゴリのID
}
```

---

## 参照

- [models.md](./models.md): モデル定義と関係性
- [database-design.md](./database-design.md): テーブル定義と制約
- [ubiquitous-language.md](./ubiquitous-language.md): ドメイン用語の定義

