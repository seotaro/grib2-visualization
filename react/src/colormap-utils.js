
export const colormaps = (master_table_number, category, number) => {
    switch (master_table_number) {
        // 0 Meteorological products
        case 0:
            switch (category) {
                // Temperature
                case 0:
                    switch (number) {
                        case 0: return COLORMAPS['temperature'];    // "Temperature [K]"
                    }
                    break;

                // Moisture
                case 1:
                    switch (number) {
                        case 1: return COLORMAPS['percentage'];                 // Relative Humidity [%]
                        case 200: return COLORMAPS['precipitation']             // 1時間降水量レベル値
                        case 201: return COLORMAPS['precipitation'];            // 10分間降水強度（１時間換算値）レベル値
                        case 202: return COLORMAPS['precipitation-10min'];      // 10分間降水量レベル値
                        case 203: return COLORMAPS['precipitation'];            // 降水強度レベル値(解析、予報）
                        case 204: return COLORMAPS['precipitation'];            // 総降水量のレベル値
                        case 206: return COLORMAPS['precipitation'];            // 土壌雨量タンクレベル値
                        case 208: return COLORMAPS['sediment-warning-index'];   // 土砂災害警戒判定値
                        case 216: return COLORMAPS['warning-index'];            // 浸水危険度判定値
                        case 217: return COLORMAPS['warning-index'];            // 洪水危険度判定値
                        case 218: return COLORMAPS['warning-index'];            // 浸水・洪水危険度判定値
                        case 233: return COLORMAPS['precipitation'];            // 降雪の深さの合計のレベル値
                    }
                    break;

                // Momentum
                case 2:
                    switch (number) {
                        case 2:   // U-Component of Wind [m s-1]
                        case 3:   // V-Component of Wind [m s-1]
                            return COLORMAPS['wind'];
                    }
                    break;

                // Mass
                case 3:
                    break;

                // Short-wave radiation
                case 4:
                    switch (number) {
                        case 51: // UV index
                        case 52: // Downward short-wave radiation flux, clear sky
                            return COLORMAPS['un-index'];
                    }
                    break;

                // Cloud
                case 6:
                    switch (number) {
                        case 1: // Total Cloud Cover [%]
                        case 3: // Low Cloud Cover [%]
                        case 4: // Medium Cloud Cover [%]
                        case 5: // High Cloud Cover [%]
                            return COLORMAPS['cloud'];
                    }
                    break;

                case 19:
                    switch (number) {
                        case 0: return COLORMAPS['visibility']; // Visibility
                    }
                    break;

                // Miscellaneous
                case 191:
                    switch (number) {
                        case 192: return COLORMAPS['weather'];  // 天気
                    }
                    break;



                // ナウキャスト
                case 193:
                    switch (number) {
                        case 0: return COLORMAPS['tornado'];    // 竜巻発生確度
                        case 1: return COLORMAPS['thunder'];    // 雷活動度
                    }
                    break;
            }
            break;

        // Oceanographic products
        case 10:
            switch (category) {
                // surface properties
                case 3:
                    switch (number) {
                        case 0: return COLORMAPS['temperature'];    // "Water temperature [K]"

                        case 200:// 天文潮位
                        case 201:// 実際の潮位
                            return COLORMAPS['tide'];
                    }
                    break;
            }
            break;
    }


    return null;
}

export const MAX_COLORMAP_STEP = 100; // GLSL の for ループのインデックスは定数値しか比較できないので固定サイズにする。

export const createGrayscaleColormap = (min, max, steps) => {
    const colors = [];
    const thresholds = [];
    let i = 0;
    for (let i = 0; i < steps; i++) {
        const d = i / (steps - 1);
        thresholds.push(min + (max - min) * d);
        colors.push([d, d, d, 1.0]);
    }
    return { thresholds, colors };
}

export const createRainbowColormap = (min, max, steps) => {
    const colors = [];
    const thresholds = [];
    let i = 0;
    for (let i = 0; i < steps; i++) {
        const d = i / (steps - 1);
        const threshold = min + (max - min) * d;

        let H = 0.0;
        if (min < max) {
            H = (1.0 - ((threshold - min) / (max - min))) * 240.0;
            if (H < 0.0) {
                H = 0.0;
            }
        }
        let S = 1.0;
        let V = 1.0;

        const RGB = HSVtoRGB(H, S, V);
        thresholds.push(threshold);
        colors.push([RGB[0], RGB[1], RGB[2], 1.0]);
    }
    return { thresholds, colors };
}

export const normalizeRange = ({ min, max }) => {
    const d = max - min;
    if (d < 0.0000001) {
        min = Math.ceil(min * 100000000.0) / 100000000.0;
        max = Math.floor(max * 100000000.0) / 100000000.0;
    } else if (d < 0.000001) {
        min = Math.ceil(min * 10000000.0) / 10000000.0;
        max = Math.floor(max * 10000000.0) / 10000000.0;
    } else if (d < 0.00001) {
        min = Math.ceil(min * 1000000.0) / 1000000.0;
        max = Math.floor(max * 1000000.0) / 1000000.0;
    } else if (d < 0.0001) {
        min = Math.ceil(min * 100000.0) / 100000.0;
        max = Math.floor(max * 100000.0) / 100000.0;
    } else if (d < 0.001) {
        min = Math.ceil(min * 10000.0) / 10000.0;
        max = Math.floor(max * 10000.0) / 10000.0;
    } else if (d < 0.01) {
        min = Math.ceil(min * 1000.0) / 1000.0;
        max = Math.floor(max * 1000.0) / 1000.0;
    } else if (d < 0.1) {
        min = Math.ceil(min * 100.0) / 100.0;
        max = Math.floor(max * 100.0) / 100.0;
    } else if (d < 1.0) {
        min = Math.ceil(min * 10.0) / 10.0;
        max = Math.floor(max * 10.0) / 10.0;
    } else {
        min = Math.ceil(min);
        max = Math.floor(max);
    }
    return { min, max };
}

// H: Hue angle
// S: Saturation
// V: Value
const HSVtoRGB = (H, S, V) => {
    if (360.0 <= H) {
        H = 0.0;
    }

    const Hi = Math.floor(H / 60.0) % 6;
    const f = H / 60.0 - Hi;
    const p = V * (1.0 - S);
    const q = V * (1.0 - S * f);
    const t = V * (1.0 - S * (1.0 - f));

    let RGB = [0.0, 0.0, 0.0];
    switch (Hi) {
        case 0: RGB = [V, t, p]; break;
        case 1: RGB = [q, V, p]; break;
        case 2: RGB = [p, V, t]; break;
        case 3: RGB = [p, q, V]; break;
        case 4: RGB = [t, p, V]; break;
        case 5: RGB = [V, p, q]; break;
    }
    return RGB;
}

const createColormaps = () => {
    {
        // temperature
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(-10.0 + 273.15); colors.push([0.0, 29 / 255, 114 / 255, 1.0]);
        thresholds.push(-5.0 + 273.15); colors.push([0.0, 57 / 255, 248 / 255, 1.0]);
        thresholds.push(0.0 + 273.15); colors.push([0.0, 139 / 255, 250 / 255, 1.0]);
        thresholds.push(5.0 + 273.15); colors.push([169 / 255, 232 / 255, 253 / 255, 1.0]);
        thresholds.push(10.0 + 273.15); colors.push([255 / 255, 255 / 255, 239 / 255, 1.0]);
        thresholds.push(15.0 + 273.15); colors.push([255 / 255, 255 / 255, 148 / 255, 1.0]);
        thresholds.push(20.0 + 273.15); colors.push([252 / 255, 243 / 255, 55 / 255, 1.0]);
        thresholds.push(25.0 + 273.15); colors.push([255 / 255, 143 / 255, 39 / 255, 1.0]);
        thresholds.push(30.0 + 273.15); colors.push([255 / 255, 38 / 255, 27 / 255, 1.0]);
        thresholds.push(35.0 + 273.15); colors.push([180 / 255, 8 / 255, 92 / 255, 1.0]);



        colormaps['temperature'] = { thresholds, colors };
    }

    {
        // percentage
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0.1, 0.1, 0.1, 1.0]);
        thresholds.push(10.0); colors.push([0.2, 0.2, 0.2, 1.0]);
        thresholds.push(20.0); colors.push([0.3, 0.3, 0.3, 1.0]);
        thresholds.push(30.0); colors.push([0.4, 0.4, 0.4, 1.0]);
        thresholds.push(40.0); colors.push([0.5, 0.5, 0.5, 1.0]);
        thresholds.push(50.0); colors.push([0.6, 0.6, 0.6, 1.0]);
        thresholds.push(60.0); colors.push([0.7, 0.7, 0.7, 1.0]);
        thresholds.push(70.0); colors.push([0.8, 0.8, 0.8, 1.0]);
        thresholds.push(80.0); colors.push([0.9, 0.9, 0.9, 1.0]);
        thresholds.push(90.0); colors.push([1.0, 1.0, 1.0, 1.0]);
        colormaps['percentage'] = { thresholds, colors };
    }

    {
        // cloud
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0.0, 0.0, 0.0, 1.0]);
        thresholds.push(10.0); colors.push([0.2, 0.2, 0.2, 1.0]);
        thresholds.push(20.0); colors.push([0.3, 0.3, 0.3, 1.0]);
        thresholds.push(30.0); colors.push([0.4, 0.4, 0.4, 1.0]);
        thresholds.push(40.0); colors.push([0.5, 0.5, 0.5, 1.0]);
        thresholds.push(50.0); colors.push([0.6, 0.6, 0.6, 1.0]);
        thresholds.push(60.0); colors.push([0.7, 0.7, 0.7, 1.0]);
        thresholds.push(70.0); colors.push([0.8, 0.8, 0.8, 1.0]);
        thresholds.push(80.0); colors.push([0.9, 0.9, 0.9, 1.0]);
        thresholds.push(90.0); colors.push([1.0, 1.0, 1.0, 1.0]);
        colormaps['cloud'] = { thresholds, colors };
    }

    {
        // precipitation
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0.0, 0.0, 0.0, 0.0]);    // No echo
        thresholds.push(0.1); colors.push([240 / 255, 240 / 255, 254 / 255, 1.0]);
        thresholds.push(1.0); colors.push([153 / 255, 204 / 255, 253 / 255, 1.0]);
        thresholds.push(5.0); colors.push([44 / 255, 131 / 255, 251 / 255, 1.0]);
        thresholds.push(10.0); colors.push([27 / 255, 65 / 255, 250 / 255, 1.0]);
        thresholds.push(20.0); colors.push([253 / 255, 241 / 255, 49 / 255, 1.0]);
        thresholds.push(30.0); colors.push([251 / 255, 143 / 255, 36 / 255, 1.0]);
        thresholds.push(50.0); colors.push([250 / 255, 46 / 255, 28 / 255, 1.0]);
        thresholds.push(80.0); colors.push([168 / 255, 23 / 255, 93 / 255, 1.0]);
        colormaps['precipitation'] = { thresholds, colors };
    }

    {
        // 10分間降水強度
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0 / 6.0); colors.push([0.0, 0.0, 0.0, 0.0]);    // No echo
        thresholds.push(0.1 / 6.0); colors.push([240 / 255, 240 / 255, 254 / 255, 1.0]);
        thresholds.push(1.0 / 6.0); colors.push([153 / 255, 204 / 255, 253 / 255, 1.0]);
        thresholds.push(5.0 / 6.0); colors.push([44 / 255, 131 / 255, 251 / 255, 1.0]);
        thresholds.push(10.0 / 6.0); colors.push([27 / 255, 65 / 255, 250 / 255, 1.0]);
        thresholds.push(20.0 / 6.0); colors.push([253 / 255, 241 / 255, 49 / 255, 1.0]);
        thresholds.push(30.0 / 6.0); colors.push([251 / 255, 143 / 255, 36 / 255, 1.0]);
        thresholds.push(50.0 / 6.0); colors.push([250 / 255, 46 / 255, 28 / 255, 1.0]);
        thresholds.push(80.0 / 6.0); colors.push([168 / 255, 23 / 255, 93 / 255, 1.0]);
        colormaps['precipitation-10min'] = { thresholds, colors };
    }

    {
        // total precipitation
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0.0, 0.0, 0.0, 0.0]);
        thresholds.push(0.1); colors.push([240 / 255, 240 / 255, 254 / 255, 1.0]);
        thresholds.push(50.0); colors.push([153 / 255, 204 / 255, 253 / 255, 1.0]);
        thresholds.push(80.0); colors.push([44 / 255, 131 / 255, 251 / 255, 1.0]);
        thresholds.push(100.0); colors.push([27 / 255, 65 / 255, 250 / 255, 1.0]);
        thresholds.push(150.0); colors.push([253 / 255, 241 / 255, 49 / 255, 1.0]);
        thresholds.push(200.0); colors.push([251 / 255, 143 / 255, 36 / 255, 1.0]);
        thresholds.push(250.0); colors.push([250 / 255, 46 / 255, 28 / 255, 1.0]);
        thresholds.push(300.0); colors.push([168 / 255, 23 / 255, 93 / 255, 1.0]);
        colormaps['total-precipitation'] = { thresholds, colors };
    }

    {
        // precipitation level
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([240 / 255, 240 / 255, 254 / 255, 1.0]);
        thresholds.push(1.0); colors.push([153 / 255, 204 / 255, 253 / 255, 1.0]);
        thresholds.push(5.0); colors.push([44 / 255, 131 / 255, 251 / 255, 1.0]);
        thresholds.push(10.0); colors.push([27 / 255, 65 / 255, 250 / 255, 1.0]);
        thresholds.push(20.0); colors.push([253 / 255, 241 / 255, 49 / 255, 1.0]);
        thresholds.push(30.0); colors.push([251 / 255, 143 / 255, 36 / 255, 1.0]);
        thresholds.push(50.0); colors.push([250 / 255, 46 / 255, 28 / 255, 1.0]);
        thresholds.push(80.0); colors.push([168 / 255, 23 / 255, 93 / 255, 1.0]);
        colormaps['precipitation-level'] = { thresholds, colors };
    }

    {
        // wind
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([240 / 255, 240 / 255, 254 / 255, 1.0]);
        thresholds.push(5.0); colors.push([0 / 255, 57 / 255, 248 / 255, 1.0]);
        thresholds.push(10.0); colors.push([252 / 255, 243 / 255, 55 / 255, 1.0]);
        thresholds.push(15.0); colors.push([255 / 255, 143 / 255, 39 / 255, 1.0]);
        thresholds.push(20.0); colors.push([255 / 255, 38 / 255, 27 / 255, 1.0]);
        thresholds.push(25.0); colors.push([180 / 255, 8 / 255, 92 / 255, 1.0]);
        colormaps['wind'] = { thresholds, colors };
    }

    {
        // weather
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0 / 255, 0 / 255, 0 / 255, 1.0]);
        thresholds.push(1.0); colors.push([255 / 255, 208 / 255, 148 / 255, 1.0]);
        thresholds.push(2.0); colors.push([208 / 255, 208 / 255, 208 / 255, 1.0]);
        thresholds.push(3.0); colors.push([128 / 255, 156 / 255, 252 / 255, 1.0]);
        thresholds.push(4.0); colors.push([198 / 255, 229 / 255, 254 / 255, 1.0]);
        thresholds.push(5.0); colors.push([247 / 255, 247 / 255, 255 / 255, 1.0]);
        colormaps['weather'] = { thresholds, colors };
    }

    {
        // 竜巻発生確度
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);          // 計算領域外又は欠測
        thresholds.push(1.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);          // なし
        thresholds.push(2.0); colors.push([251 / 255, 248 / 255, 151 / 255, 1.0]);    // 発生確度1
        thresholds.push(3.0); colors.push([252 / 255, 150 / 255, 141 / 255, 1.0]);    // 発生確度2
        colormaps['tornado'] = { thresholds, colors };
    }

    {
        // 雷活動度
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);          // 計算領域外又は欠測
        thresholds.push(1.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);          // なし
        thresholds.push(2.0); colors.push([254 / 255, 248 / 255, 151 / 255, 1.0]);    // 活動度1
        thresholds.push(3.0); colors.push([253 / 255, 207 / 255, 146 / 255, 1.0]);    // 活動度2
        thresholds.push(4.0); colors.push([252 / 255, 150 / 255, 141 / 255, 1.0]);    // 活動度3
        thresholds.push(5.0); colors.push([223 / 255, 147 / 255, 252 / 255, 1.0]);    // 活動度4
        colormaps['thunder'] = { thresholds, colors };
    }

    {
        // 土砂災害警戒判定値
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 欠測値
        thresholds.push(1.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 海等の格子
        thresholds.push(2.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 警戒判定対象外格子
        thresholds.push(3.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 土砂災害警戒判定値0
        thresholds.push(4.0); colors.push([242 / 255, 231 / 255, 0 / 255, 1.0]);  // 土砂災害警戒判定値1
        thresholds.push(5.0); colors.push([255 / 255, 40 / 255, 0 / 255, 1.0]);   // 土砂災害警戒判定値2
        thresholds.push(6.0); colors.push([170 / 255, 0 / 255, 170 / 255, 1.0]);  // 土砂災害警戒判定値3
        thresholds.push(7.0); colors.push([12 / 255, 0 / 255, 12 / 255, 1.0]);    // 土砂災害警戒判定値4
        thresholds.push(8.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 予備 
        thresholds.push(9.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 予備 
        thresholds.push(10.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);     // 予備 
        colormaps['sediment-warning-index'] = { thresholds, colors };
    }

    {
        // 警戒判定値
        const colors = [];
        const thresholds = [];
        let i = 0;
        thresholds.push(0.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 欠測値
        thresholds.push(1.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 土砂災害警戒判定値0
        thresholds.push(2.0); colors.push([242 / 255, 231 / 255, 0 / 255, 1.0]);  // 土砂災害警戒判定値1
        thresholds.push(3.0); colors.push([255 / 255, 40 / 255, 0 / 255, 1.0]);   // 土砂災害警戒判定値2
        thresholds.push(4.0); colors.push([170 / 255, 0 / 255, 170 / 255, 1.0]);  // 土砂災害警戒判定値3
        thresholds.push(5.0); colors.push([12 / 255, 0 / 255, 12 / 255, 1.0]);    // 土砂災害警戒判定値4
        thresholds.push(6.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 予備 
        thresholds.push(7.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 予備 
        thresholds.push(8.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 予備 
        thresholds.push(9.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);      // 予備 
        thresholds.push(10.0); colors.push([0 / 255, 0 / 255, 0 / 255, 0.0]);     // 予備 
        colormaps['warning-index'] = { thresholds, colors };
    }

    {
        // UV index
        colormaps['uv-index'] = createRainbowColormap(0, 13, 14);
    }

    // short-wave radiation flux
    colormaps['short-wave-radiation-flux'] = createGrayscaleColormap(0.0, 1000.0, 10);

    // pressure
    colormaps['pressure'] = createGrayscaleColormap(95000.0, 105000.0, 10);

    // Vertical Velocity (Pressure)
    colormaps['vertical-velocity'] = createGrayscaleColormap(0.0, 1000.0, 10);

    // Geopotential Height
    colormaps['geopotential-height'] = createGrayscaleColormap(0.0, 1000.0, 10);

    // Tide
    colormaps['tide'] = createRainbowColormap(0.6, 3, 24);

    // Visibility
    colormaps['visibility'] = createRainbowColormap(0.0, 10000.0, 20);

    {
        // sample
        const colors = [];
        const thresholds = [];
        for (let i = 0; i < MAX_COLORMAP_STEP; i++) {
            thresholds.push(i); colors.push([i / (MAX_COLORMAP_STEP - 1), 0.0, i / (MAX_COLORMAP_STEP - 1, 1.0)], i * 4);
        }
        colormaps['sample'] = { thresholds, colors };
    }

    return colormaps;
}

const COLORMAPS = createColormaps();
