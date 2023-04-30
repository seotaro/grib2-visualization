# GRIB2 Parser and Visualization

## Demo

[Live Demo](https://seotaro.github.io/grib2-rust/)

![image](https://user-images.githubusercontent.com/46148606/235303415-89b75629-29dc-4148-b560-3098636c0624.png)

## Notes

|  product  |  version |  template3.  |  template4.  |  template5.  |  Bit map  | template7.  | Packing  |
| ---- | ----  | ---- | ---- | ---- | ---- | ---- | ---- |
|  GSM（全球域）  |      |  0  |  0/8  |  0  |  -   | 0  | simple  |
|  GSM（日本域）  |  2023/03〜    |  0  |  0/8  |  3  |  -   | 3  | complex packing and spatial differencing  |
|  LFM  |      |  0  |  0/8  |  0  |  applied   | 0  | simple  |
|  5分毎1kmメッシュ全国合成レーダー  |      |  0  |  50008  |  200  |  -  | 200  | run length  |
|  高解像度降水ナウキャスト  |      |  0  |  50011  |  200  |  -  | 200  | run length  |

## References

- ドキュメント
  - [WMO](https://public.wmo.int/) >> [Manual on Codes](https://library.wmo.int/index.php?lvl=notice_display&id=10684#.ZEsjTOxBz0p)
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [気象通報式](https://www.jma.go.jp/jma/kishou/books/tsuhoshiki/tsuhoshiki.html)
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [気象庁情報カタログ](https://www.data.jma.go.jp/add/suishin/catalogue/catalogue.html)
  - [気象庁](https://www.jma.go.jp/jma/index.html) >> [配信資料に関する技術情報](https://www.data.jma.go.jp/add/suishin/cgi-bin/jyouhou/jyouhou.cgi)

- データ
  - [（一財）気象業務支援センター](http://www.jmbsc.or.jp/jp/index.html) >> [オンライン気象情報](http://www.jmbsc.or.jp/jp/index.html#gaiyou-online) >> [ファイル形式データ](http://www.jmbsc.or.jp/jp/online/f-online0.html)
  - [京都大学生存圏研究所](http://database.rish.kyoto-u.ac.jp/) >> [グローバル大気観測データ](http://database.rish.kyoto-u.ac.jp/arch/glob-atmos/) >> [気象庁データ ※](http://database.rish.kyoto-u.ac.jp/arch/jmadata/gpv-original.html)

※ 京都大学生存圏研究所がホストされている気象庁データの利用上の注意
> ここでは教育研究機関向けにデータを提供しています．企業活動等のためにデータを頻繁に必要とされる方は，気象業務支援センターからデータを直接購入し，データ提供スキーム全体の維持発展にご協力ください．
