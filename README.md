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

|                             product                              |  version  | template3. | template4. | template5. | Bit map | template7. |                 Packing                  |
| ---------------------------------------------------------------- | --------- | ---------- | ---------- | ---------- | ------- | ---------- | ---------------------------------------- |
| GSM（全球域）                                                    |           | 0          | 0/8        | 0          | -       | 0          | simple                                   |
| GSM（日本域）                                                    | 2023/03〜 | 0          | 0/8        | 3          | -       | 3          | complex packing and spatial differencing |
| LFM                                                              |           | 0          | 0/8        | 0          | applied | 0          | simple                                   |
| 5分毎1kmメッシュ全国合成レーダー                                 |           | 0          | 50008      | 200        | -       | 200        | run length                               |
| 高解像度降水ナウキャスト                                         |           | 0          | 50011      | 200        | -       | 200        | run length                               |
| GSMガイダンス (格子形式)                                         |           |            |            |            |         |            |                                          |
| GSMガイダンス (最大降水量、降雪量)                               |           | 0          | 8          | 0          | applied | 0          | simple                                   |
| GSMガイダンス (視程)                                             |           | 0          | 8          | 0          | -       | 0          | simple                                   |
| MSM                                                              |           | 0          | 0/8        | 0          | -       | 0          | simple                                   |
| MSMガイダンス (格子形式)                                         |           | 0          | 8/9        | 0          | applied | 0          | simple                                   |
| MSMガイダンス (最大降水量)                                       |           | 0          | 8          | 0          | applied | 0          | simple                                   |
| MSM大雨発生確率ガイダンス                                        |           | 0          | 9          | 0          | applied | 0          | simple                                   |
| 週間アンサンブル数値予報モデルGPV (全球域)                       |           | 0          | 1/11       | 3          | -       | 3          | complex packing and spatial differencing |
| 週間アンサンブル数値予報モデルGPV (日本域)                       |           | 0          | 1/11       | 3          | -       | 3          | complex packing and spatial differencing |
| 台風アンサンブル数値予報モデルGPV (日本域)                       |           | 0          | 1/11       | 3          | -       | 3          | complex packing and spatial differencing |
| メソアンサンブル予報システム（ＭＥＰＳ）GPV                      |           | 0          | 1/11       | 3          | -       | 3          | complex packing and spatial differencing |
| メソアンサンブルガイダンス (格子形式)                            |           | 0          | 11         | 3          | -       | 3          | complex packing and spatial differencing |
| メソアンサンブルガイダンス (最大降水量、降雪量)                  |           | 0          | 11         | 3          | -       | 3          | complex packing and spatial differencing |
| MEPS大雨発生確率ガイダンス                                       |           | 0          | 9          | 0          | applied | 0          | simple                                   |
| 三十分大気解析                                                   |           | 0          | 0          | 3          | applied | 3          | complex packing and spatial differencing |
| 降水ナウキャスト (５分)                                          |           | 0          | 50008      | 200        | -       | 200        | run length                               |
| 竜巻発生確度ナウキャスト                                         |           | 0          | 0          | 200        | -       | 200        | run length                               |
| 雷ナウキャスト                                                   |           | 0          | 0          | 200        | -       | 200        | run length                               |
| 1kmメッシュ解析雨量                                              |           | 0          | 50008      | 200        | -       | 200        | run length                               |
| 降水短時間予報GPV                                                |           | 0          | 50009      | 200        | -       | 200        | run length                               |
| 速報版解析雨量                                                   |           | 0          | 50008      | 200        | -       | 200        | run length                               |
| 速報版降水短時間予報                                             |           | 0          | 50009      | 200        | -       | 200        | run length                               |
| 降水15時間予報                                                   |           | 0          | 50012      | 200        | -       | 200        | run length                               |
| 解析積雪深                                                       |           | 0          | 0          | 200        | -       | 200        | run length                               |
| 解析降雪量                                                       |           | 0          | 8          | 200        | -       | 200        | run length                               |
| 土壌雨量指数                                                     |           | 0          | 0          | 200        | -       | 200        | run length                               |
| 大雨警報（土砂災害）の危険度分布（土砂災害警戒判定メッシュ情報） |           | 0          | 50000      | 200        | -       | 200        | run length                               |

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
