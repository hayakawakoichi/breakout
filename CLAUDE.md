# Breakout Game

Rust + Bevy 0.15 で作成したブロック崩しゲーム。

## Tech Stack

- **言語**: Rust (edition 2021)
- **ゲームエンジン**: Bevy 0.15
- **アーキテクチャ**: ECS (Entity Component System)

## ビルド・実行

```bash
cargo build
cargo run
```

- 初回ビルドには約3GB以上のディスク空き容量が必要
- Rust未インストールの場合: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## 操作方法

| キー | 操作 |
|------|------|
| SPACE | ゲーム開始 / リスタート / 次レベル |
| ← → / A D | パドル移動 |
| ESC | 一時停止 / 再開 |

## プロジェクト構造

```
src/
├── main.rs           # エントリーポイント、App設定・システム登録
├── components.rs     # ECSコンポーネント (Paddle, Ball, Block, Wall, Collider等)
├── resources.rs      # リソース (Score, Level, GameSounds)
├── constants.rs      # ゲーム定数 (画面サイズ、速度、ブロック配置等)
├── states.rs         # ゲーム状態Enum (Menu, Playing, Paused, GameOver, LevelClear)
└── systems/
    ├── mod.rs        # システムモジュールの公開
    ├── setup.rs      # 初期化 (カメラ、パドル、ボール、ブロック、壁、UI生成)
    ├── input.rs      # 入力処理 (パドル移動、ゲーム開始、一時停止)
    ├── movement.rs   # ボール移動
    ├── collision.rs  # 衝突検出 (パドル/壁/ブロック、勝利判定)
    ├── scoring.rs    # スコア・レベル表示更新
    ├── audio.rs      # サウンド再生 (CollisionEvent)
    └── game_state.rs # 状態管理 (メニュー/ゲームオーバー/レベルクリア画面)
```

## Bevy 0.15 注意点

- `SpriteBundle`, `Camera2dBundle`, `TextBundle` は廃止済み
  - `Sprite` + `Transform` を直接使用
  - `Camera2d` のみでカメラ生成
  - `Text` + `Node` でUI表示
- `AudioPlayer::new(source)` でサウンド再生
- `ButtonInput<KeyCode>` でキーボード入力取得
- サブモジュールの型を main.rs で使う場合は明示的にインポートが必要
  - 例: `use components::Block;`

## ゲーム仕様

- **ブロック**: 5行 x 10列 (50個)、行ごとに色分け
- **スコア**: ブロック破壊で10点
- **レベル**: クリア後にレベル上昇、ボール速度が10%/レベル増加
- **サウンド**: `assets/sounds/` に .ogg ファイルを配置（無くても動作可）
- **衝突**: AABB判定、パドル当たり位置でボール反射角度が変化

## モバイル対応

- **カメラスケーリング**: `ScalingMode::AutoMin` で最低800x600のゲーム領域を常に表示（ポートレートでは上下余白）
- **UIスケーリング**: `UiScale` リソースを `(window_width / 800).clamp(0.5, 1.0)` で毎フレーム更新し、フォント・余白を比例縮小
- **タッチ入力**: `camera.viewport_to_world_2d()` でスクリーン座標→ワールド座標に正しく変換
