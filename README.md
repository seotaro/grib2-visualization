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
