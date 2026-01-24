グランドステージ: 2.The Stack

フェーズ: BOSS Stage

ステージ名: The Stack Machine

ステージ内容: スタックを使ってトークン列を評価する簡易RPN計算機を作る（総合問題）

入力仕様:
- 0: 終了
- 正の値: push
- 負の値: 命令
  - -1: ADD（a + b）
  - -2: SUB（a - b） ※順序注意（先にpopした方が b）
  - -3: XOR（a ^ b）

出力仕様:
- 終了時点のスタックtopを sys_write で出力する

