# 自己紹介
- 2019年3月に筑波大学大学院をこんぶくんと一緒に修了
- 2019年4月よりハンズラボでiOSエンジニアとして働く
- 2020年にRustを出会い学びはじめる
  - 社内でRustの勉強会を開き、Rustを広めてプロダクトで使ってもらおうと努力するも、社内のエンジニア達のスキルとマッチしないため断念。Rustを採用してる会社へ転職するべく動き出す
- 2022年2月よりHandii（現ペイルド）で働き始める
- 2023年9月にアメリカのアルバカーキで開催されたRustConfに現地参加
  - [RustConf2023参加記｜note](https://note.com/ryosukeeeee_/n/nae89cf7e047e)


## 興味

### 形式手法（formal method）

- 形式仕様記述
  - VDM-SL, VDM++, Z
- 形式検証
- モデル検査
  - TLA+, Alloy, SPIN

自然言語による仕様は曖昧になりがち

ex. 配列`v`の中で値`x`の後ろに`y`がある場合はtrue、ない場合はfalseを返す関数を作ってください

こんなときはどうする？

- 配列の長さが0
- xが複数
- xが配列に存在しない  
- xが末尾にある
- x = y だったら？

### 形式的な記述の例

数理論理学に基づいた仕様記述

$$\forall i. 0\le i  \lt n-1 \land v[i] = x \Rightarrow v[i+1] = y$$

- 配列の長さが0のときtrue
- xが複数のとき、それぞれのxの次のインデックスの要素がyならtrue
  - OK: [x, y, x, y]
  - NG: [x, y, x, 2]
- xが配列に存在しないときtrue
- xが末尾にあるときtrue
- x = y のときは、配列の先頭から辿って最初のx以降がすべてxならtrue
  - [0, 4, 5, x, x, x, x]

「配列`v`の中で値`x`の後ろに`y`がある場合はtrue、ない場合はfalseを返す関数を作ってください」の別の形式的な記述もありうる。例えば、

$$\forall i. 0 \le i \lt n-1 \land v[i] = x \Rightarrow \exist j. i \lt j \lt n \land v[j] = y $$


## 自作のモデル検査器

[ryosukeeeee/ddsv](https://github.com/ryosukeeeee/ddsv/tree/develop) <br>
これはRustを学び始めた初期の頃に書いたプログラム。OCamlで書かれたプログラムを素直にRustで書き直したので、Rustっぽくない書き方になっているかも。


例えば、以下のような状態遷移をする2つのプロセスPとQを考える。

- Pは先に0番のロックを取得してから1番のロックを取得、その後1番、0番の順にロックを解放する。
- Qは先に1番のロックを取得してから0番のロックを取得、その後0番、1番の順にロックを解放する。

<table border="1">
  <tr>
    <th>Pの状態遷移（状態数4）</th>
    <th>Qの状態遷移（状態数4）</th>
  </tr>
  <tr>
    <td><img src=../medias/m_mutex2_P.svg></td>
    <td><img src=../medias/m_mutex2_Q.svg></td>
  </tr>
</table>

プロセスPとQを合成するとこのような状態遷移図になる（状態数9）。
水色が初期状態。赤色がデッドロック（赤色から始まる遷移がないので）

<img src=../medias/m_mutex2.svg>

### 参考
[形式手法 - Wikipedia](https://ja.wikipedia.org/wiki/%E5%BD%A2%E5%BC%8F%E6%89%8B%E6%B3%95)
[ゼロから学んだ形式手法 - DeNA Testing Blog](https://swet.dena.com/entry/2020/04/08/140500)

