# yew PWA example
yew + trunkでPWAを作るときのメモ．

# targetフォルダの移動
`.cargo/config.toml`に以下を追記
```toml
[build]
target-dir = ".output/target" #Cargo.tomlからの相対パス
```

# index.html の移動

- `Trunk.toml`に以下を追記．
  ```toml
  [build]
  target = "src/wwwroot/index.html" #Trunk.tomlからindex.htmlまでの相対パス
  ```

- `index.html`に以下を記述．
  `href`には`index.html`から`Cargo.toml`への相対パスを記述．
  `index.html`をルートから移動した場合，これを書かないと動かない．

  ```html
  <link data-trunk rel="rust" href="../../Cargo.toml" />
  ```

# distの移動
`Trunk.toml`に以下を追記．
```toml
[build]
dist = ".output/dist"
```

# Service Worker

- Service Workerはプロキシとして動く．

- Service Workerはスコープを指定できるが，置かれているディレクトリ以下だけに対してのみ有効．


- PWAの更新はService Workerの更新によって検出される．
  バージョンを表す変数を以下のように書いておくとよい．
  ```js
  var version = 'yew-trial 0.0.0';
  ```

# trunkのhook
trunkはビルド時，中間生成ファイルを`dist`の`.stage`サブフォルダにおき，その後，`.stage`の中身を`dist`にコピーする．

そのため，hookの中で生成物の内容を書き換えたいなら`.stage`内の生成物を書き換える．

# manifest
これが置かれているディレクトリがWeb APIのファイルパスの基準になったりする．サイトのrootに置いておくとよい．

拡張子は`json`, `webmanifest`の複数があるが，どちらでもよい．

ここに記述される`scope`と一致するスコープを持つService Workerが必要．
