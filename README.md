# Parse and visualize GRIB2 data

## Demo

[Live Demo](https://seotaro.github.io/grib2-visualization/)

<img width="500" alt="image" src="https://user-images.githubusercontent.com/46148606/235341919-ed067400-c9c6-415c-b926-5534f29b75c3.png">

## Install

```bash
make Install
```

## Build

```bash
make build-for-wasm
```

## run

```bash
make run
```

## Notes on JWA GIRB2

### Filename

Naming rules

- Region: R{gl|jp}
- Levels: L{surf|-pall|pLLL}
- Forecasts: FD{DDhh}
- Resolution: {n}p{n}deg

### Template, Packing

|                     product                     |  version  | 3.  |    4.     |  5.   |   6.    |  7.   |                 Packing                  |
| ----------------------------------------------- | --------- | --- | --------- | ----- | ------- | ----- | ---------------------------------------- |
| GSM（全球域）                                   |           | 3.0 | 4.0, 4.8  | 5.0   | -       | 7.0   | simple                                   |
| GSM（日本域）                                   | 2023/03〜 | 3.0 | 4.0, 4.8  | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| GSMガイダンス (格子形式)                        |           | 3.0 | 4.8, 4.9  | 5.0   | Bit map | 7.0   | simple                                   |
| GSMガイダンス (最大降水量、降雪量)              |           | 3.0 | 4.8       | 5.0   | Bit map | 7.0   | simple                                   |
| GSMガイダンス (視程)                            |           | 3.0 | 4.8       | 5.0   | -       | 7.0   | simple                                   |
| MSM                                             |           | 3.0 | 4.0, 4.8  | 5.0   | -       | 7.0   | simple                                   |
| MSMガイダンス (格子形式)                        |           | 3.0 | 4.8, 4.9  | 5.0   | Bit map | 7.0   | simple                                   |
| MSMガイダンス (最大降水量)                      |           | 3.0 | 4.8       | 5.0   | Bit map | 7.0   | simple                                   |
| MSM大雨発生確率ガイダンス                       |           | 3.0 | 4.9       | 5.0   | Bit map | 7.0   | simple                                   |
| LFM                                             |           | 3.0 | 4.0, 4.8  | 5.0   | Bit map | 7.0   | simple                                   |
| 週間アンサンブル数値予報モデルGPV (全球域)      |           | 3.0 | 4.1, 4.11 | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| 週間アンサンブル数値予報モデルGPV (日本域)      |           | 3.0 | 4.1, 4.11 | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| 台風アンサンブル数値予報モデルGPV (日本域)      |           | 3.0 | 4.1, 4.11 | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| メソアンサンブル予報システム（ＭＥＰＳ）GPV     |           | 3.0 | 4.1, 4.11 | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| メソアンサンブルガイダンス (格子形式)           |           | 3.0 | 4.11      | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| メソアンサンブルガイダンス (最大降水量、降雪量) |           | 3.0 | 4.11      | 5.3   | -       | 7.3   | complex packing and spatial differencing |
| MEPS大雨発生確率ガイダンス                      |           | 3.0 | 4.9       | 5.0   | Bit map | 7.0   | simple                                   |
| 三十分大気解析                                  |           | 3.0 | 4.0       | 5.3   | Bit map | 7.3   | complex packing and spatial differencing |
| 降水ナウキャスト (５分)                         |           | 3.0 | 4.50008   | 5.200 | -       | 7.200 | run length                               |
| 竜巻発生確度ナウキャスト                        |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 雷ナウキャスト                                  |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 5分毎1kmメッシュ全国合成レーダー                |           | 3.0 | 4.50008   | 5.200 | -       | 7.200 | run length                               |
| 高解像度降水ナウキャスト                        |           | 3.0 | 4.50011   | 5.200 | -       | 7.200 | run length                               |
| 1kmメッシュ解析雨量                             |           | 3.0 | 4.50008   | 5.200 | -       | 7.200 | run length                               |
| 降水短時間予報GPV                               |           | 3.0 | 4.50009   | 5.200 | -       | 7.200 | run length                               |
| 速報版解析雨量                                  |           | 3.0 | 4.50008   | 5.200 | -       | 7.200 | run length                               |
| 速報版降水短時間予報                            |           | 3.0 | 4.50009   | 5.200 | -       | 7.200 | run length                               |
| 降水15時間予報                                  |           | 3.0 | 4.50012   | 5.200 | -       | 7.200 | run length                               |
| 解析積雪深                                      |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 解析降雪量                                      |           | 3.0 | 4.8       | 5.200 | -       | 7.200 | run length                               |
| 土壌雨量指数                                    |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 大雨警報（土砂災害）の危険度分布                |           | 3.0 | 4.50000   | 5.200 | -       | 7.200 | run length                               |
| 表面雨量指数実況値                              |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 表面雨量指数1時間予測値                         |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 表面雨量指数6時間予測値                         |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 大雨警報(浸水害)の危険度分布                    |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 洪水警報の危険度分布                            |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 大雨警報(浸水害)                                |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 洪水警報の危険度分布（統合版）                  |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| ２週間アンサンブル数値予報モデルGPV (全球域)    |           | 3.0 | 4.1, 4.11 | 5.0   | -       | 7.0   | simple                                   |
| ２週間アンサンブル数値予報モデルGPV (日本域)    |           | 3.0 | 4.1, 4.11 | 5.0   | -       | 7.0   | simple                                   |
| 1か月アンサンブル数値予報モデルGPV (全球域)     |           | 3.0 | 4.1, 4.11 | 5.0   | -       | 7.0   | simple                                   |
| 1か月アンサンブル数値予報モデルGPV (日本域)     |           | 3.0 | 4.1, 4.11 | 5.0   | -       | 7.0   | simple                                   |
| ２週間アンサンブル統計GPV (全球域)              |           |     |           |       |         |       |                                          |
| 1か月アンサンブル統計GPV (全球域)               |           |     |           |       |         |       |                                          |
| ６か月アンサンブル数値予報モデルGPV             |           |     |           |       |         |       |                                          |
| 波浪モデルGPV                                   |           |     |           |       |         |       |                                          |
| 波浪アンサンブル数値予報モデルGPV               |           |     |           |       |         |       |                                          |
| GWM                                             |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| CWM                                             |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| 全球波浪数値予報モデル風浪・うねりGPV           |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| 沿岸波浪数値予報モデル風浪・うねりGPV           |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| 黄砂予測モデル格子点値                          |           |     |           |       |         |       |                                          |
| 黄砂解析予測モデル格子点値（予測値）            |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |
| 黄砂解析予測モデル格子点値（解析値）            |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |
| 紫外線解析データ                                |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| 紫外線予測データ（晴天とした場合の予測）        |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| 紫外線予測データ（天気を考慮した場合の予測）    |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| オゾン全量データ                                |           | 3.0 | 4.0       | 5.0   | Bit map | 7.0   | simple                                   |
| 推計気象分布（気温）                            |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 推計気象分布（天気）                            |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 推計気象分布（日照時間）                        |           | 3.0 | 4.0       | 5.200 | -       | 7.200 | run length                               |
| 高分解能雲情報（雲型）                          |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |
| 高分解能雲情報（雲頂高度）                      |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |
| 高分解能雲情報（品質情報）                      |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |
| 高分解能雲情報（雲の有無）                      |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |
| 高分解能雲情報（雪氷の有無）                    |           | 3.0 | 4.0       | 5.0   | -       | 7.0   | simple                                   |

## References

- ドキュメント
  - [WMO](https://public.wmo.int/) >> [Manual on Codes](https://library.wmo.int/index.php?lvl=notice_display&id=10684#.ZEsjTOxBz0p)
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [気象通報式](https://www.jma.go.jp/jma/kishou/books/tsuhoshiki/tsuhoshiki.html)
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [気象庁情報カタログ](https://www.data.jma.go.jp/add/suishin/catalogue/catalogue.html)
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [配信資料に関する技術情報](https://www.data.jma.go.jp/add/suishin/cgi-bin/jyouhou/jyouhou.cgi)

- データ
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [知識・解説](https://www.jma.go.jp/jma/menu/menuknowledge.html) >> [気象データ高度利用ポータルサイト](https://www.data.jma.go.jp/developer/index.html) >> [GPVサンプルデータの一覧](https://www.data.jma.go.jp/developer/gpv_sample.html)
  - [京都大学生存圏研究所](http://database.rish.kyoto-u.ac.jp/) >> [グローバル大気観測データ](http://database.rish.kyoto-u.ac.jp/arch/glob-atmos/) >> [気象庁データ ※](http://database.rish.kyoto-u.ac.jp/arch/jmadata/gpv-original.html)

※ 京都大学生存圏研究所がホストされている気象庁データの利用上の注意
> ここでは教育研究機関向けにデータを提供しています．企業活動等のためにデータを頻繁に必要とされる方は，気象業務支援センターからデータを直接購入し，データ提供スキーム全体の維持発展にご協力ください．
