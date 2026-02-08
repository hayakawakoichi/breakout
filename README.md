# ブロック崩し

Rust + Bevy 0.15 で作成したファミコン風ブロック崩しゲーム。

![ゲームプレイ画面](docs/screenshot_gameplay.png)

## 技術スタック

- **言語**: Rust (edition 2021)
- **ゲームエンジン**: Bevy 0.15
- **アーキテクチャ**: ECS (Entity Component System)
- **フォント**: DotGothic16 (ピクセルフォント、Google Fonts)
- **ビジュアル**: ファミコン風レトロカラーパレット、ピクセルパーフェクト描画

## ビルド・実行

```bash
cargo build
cargo run
```

- 初回ビルドには約 3GB 以上のディスク空き容量が必要
- Rust 未インストールの場合:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## WASM ビルド (ブラウザで遊ぶ)

```bash
# 事前準備 (初回のみ)
rustup target add wasm32-unknown-unknown
cargo install trunk

# 開発サーバー起動 (localhost:8080)
trunk serve

# 本番ビルド (dist/ に出力)
trunk build --release
```

WASM 版にはピクセルアート風のローディング画面が付属しています。

## 操作方法

| キー              | 操作                                 |
| ----------------- | ------------------------------------ |
| SPACE             | ゲーム開始 / リトライ / 次のレベルへ |
| ← → / A D        | パドル移動                           |
| タッチ            | パドル移動 / 状態遷移                |
| ESC / \|\| ボタン | 一時停止（設定オーバーレイ表示）     |
| ESC / タップ      | ポーズ解除（オーバーレイ内）         |
| E                 | ステージエディタを開く（メニュー画面）|

## ステージエディタ

自分だけのステージを作って、URLで友達に共有できます。

1. メニュー画面で `[ エディタ ]` をタップ（または `E` キー）
2. 左のツールパレットでブロック種別を選択
   - **N 通常** / **D 耐久** / **S 鉄** / **E 爆発** / **× 消去**
3. 7行x10列のグリッドをクリック/タップしてブロックを配置
4. **テストプレイ** で実際にプレイして確認
5. **共有** ボタンでURLをコピー → 相手がURLを開くとステージが復元

サーバー不要 — ステージデータはURL内にBase64エンコードされます。

## ゲーム仕様

- **ブロック**: 5 行 x 10 列 (50 個)、行ごとにファミコン風カラーで色分け (コーラル / オレンジ / イエロー / グリーン / ブルー)
- **スコア**: ブロック破壊で 10 点
- **レベル**: 全ブロック破壊でクリア、次レベルではボール速度が 10% 増加
- **衝突**: AABB 判定、パドルの当たり位置でボール反射角度が変化
- **サウンド**: `assets/sounds/` に WAV ファイルを配置（無くても動作可）
- **ポーズ画面**: ESC キーまたは HUD の `||` ボタンで一時停止。画面中央に BGM・効果音の音量調整を表示。ESC / タップで再開
- **パワーアップ**: ブロック破壊時に 15% の確率でアイテムがドロップ。パドルでキャッチすると効果発動

| アイテム | 色 | 効果 | 持続時間 |
|----------|----------|------|----------|
| ワイドパドル | マゼンタ | パドル幅 1.5 倍 | 8 秒 |
| マルチボール | シアン | ボール 2 個追加 | 永続（ロストで消滅） |
| スローボール | ライム | ボール速度 0.6 倍 | 6 秒 |
| ファイアボール | オレンジ | ブロック貫通（Steelは反射） | 5 秒 |

## プロジェクト構造

```
src/
├── main.rs           # エントリーポイント、App設定・システム登録
├── components.rs     # ECSコンポーネント (Paddle, Ball, Block, Wall, Collider等)
├── resources.rs      # リソース (Score, Level, GameSounds)
├── constants.rs      # ゲーム定数 (画面サイズ、速度、ブロック配置等)
├── states.rs         # ゲーム状態Enum (Menu, Playing, Paused, GameOver, LevelClear, Editor, TestPlay)
└── systems/
    ├── mod.rs        # システムモジュールの公開
    ├── setup.rs      # 初期化 (カメラ、パドル、ボール、ブロック、壁、UI生成)
    ├── input.rs      # 入力処理 (パドル移動、ゲーム開始、一時停止)
    ├── movement.rs   # ボール移動
    ├── collision.rs  # 衝突検出 (パドル/壁/ブロック、勝利判定)
    ├── scoring.rs    # スコア・レベル表示更新
    ├── audio.rs      # サウンド再生 (CollisionEvent)
    ├── game_state.rs # 状態管理 (メニュー/ゲームオーバー/レベルクリア/ポーズ画面)
    ├── powerup.rs    # パワーアップ (ドロップ移動・取得判定・効果管理)
    └── editor.rs     # ステージエディタ (UI構築・グリッド入力・URL共有・テストプレイ)
index.html            # WASM用HTML (ローディング画面付き)
assets/
├── fonts/
│   └── DotGothic16-Regular.ttf  # ピクセルフォント (日本語対応)
└── sounds/
    ├── bounce.wav    # バウンド音
    ├── break.wav     # ブロック破壊音
    ├── gameover.wav  # ゲームオーバー音
    └── levelup.wav   # レベルアップ音
```

## ライセンス

フォント (DotGothic16) は [SIL Open Font License](https://scripts.sil.org/OFL) の下で配布されています。
