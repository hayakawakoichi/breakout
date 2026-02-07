# Breakout Game

Rust + Bevy 0.15 で作成したブロック崩しゲーム。ファミコン風レトロビジュアル。

## Tech Stack

- **言語**: Rust (edition 2021)
- **ゲームエンジン**: Bevy 0.15
- **アーキテクチャ**: ECS (Entity Component System)
- **フォント**: DotGothic16 (ピクセルフォント、SIL OFL)
- **ビジュアル**: ファミコン風レトロカラーパレット、ピクセルパーフェクト描画

## ビルド・実行

```bash
cargo build
cargo run
```

- 初回ビルドには約3GB以上のディスク空き容量が必要
- Rust未インストールの場合: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### WASM ビルド (ブラウザで遊ぶ)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve        # 開発サーバー (localhost:8080)
trunk build --release  # 本番ビルド (dist/)
```

## 操作方法

| キー | 操作 |
|------|------|
| SPACE | ゲーム開始 / リスタート / 次レベル |
| ← → / A D | パドル移動 |
| タッチ | パドル移動 / 状態遷移 |
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
index.html            # WASM用HTML (ローディング画面付き)
```

## Bevy 0.15 注意点

- `SpriteBundle`, `Camera2dBundle`, `TextBundle` は廃止済み
  - `Sprite` + `Transform` を直接使用
  - `Camera2d` のみでカメラ生成
  - `Text` + `Node` でUI表示
- `AudioPlayer::new(source)` でサウンド再生
- `ButtonInput<KeyCode>` でキーボード入力取得
- `Msaa` は Component (Resourceではない) — カメラEntityに付与
- `ImagePlugin::default_nearest()` は `bevy::render::texture::ImagePlugin` からインポート
- `FontSmoothing::None` は `bevy::text::FontSmoothing` からインポート
- サブモジュールの型を main.rs で使う場合は明示的にインポートが必要
  - 例: `use components::Block;`

## ゲーム仕様

- **ゲーム領域**: 800x800 ゲームユニット
- **ブロック**: 5行 x 10列 (50個)、行ごとにファミコン風カラー (コーラル/オレンジ/イエロー/グリーン/ブルー)
- **スコア**: ブロック破壊で10点
- **レベル**: クリア後にレベル上昇、ボール速度が10%/レベル増加
- **サウンド**: `assets/sounds/` に WAV ファイルを配置（無くても動作可）
- **衝突**: AABB判定、パドル当たり位置でボール反射角度が変化

## ビジュアル

- **レンダリング**: `ImagePlugin::default_nearest()` + `Msaa::Off` でピクセルパーフェクト描画
- **フォント**: DotGothic16 + `FontSmoothing::None`、16px倍数サイズ (16/24/48)
- **カラーパレット**: ファミコン風の暖色系レトロカラー
- **ローディング画面** (WASM): ピクセルアート風ブロック + バウンドするボール + "Now Loading..." アニメーション

## モバイル対応

- **カメラスケーリング**: `ScalingMode::AutoMin` で最低800x800のゲーム領域を常に表示
- **UIスケーリング**: `UiScale` リソースを `(window_width / 800).clamp(0.5, 1.0)` で毎フレーム更新し、フォント・余白を比例縮小
- **タッチ入力**: `camera.viewport_to_world_2d()` でスクリーン座標→ワールド座標に正しく変換
- **ローディング画面**: WASM ロード中にピクセルアートUI表示、`ResizeObserver` で完了検知→フェードアウト
