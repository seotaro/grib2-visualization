
export const colormaps = (genre, category, number) => {
    switch (genre) {
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
                        case 1: return COLORMAPS['percentage']; // Relative Humidity [%]
                        case 8: return COLORMAPS['total-precipitation'];    // Total Precipitation [kg m-2]
                        // case 52:// Total precipitation rate [kg m-2 s-1]
                        // case 57:// Total snowfall rate [m s-1]
                        case 201: return COLORMAPS['precipitation'];    // 10分間降水強度（１時間換算値）レベル値
                        case 203: return COLORMAPS['precipitation'];    // 降水強度レベル値(解析、予報）
                    }
                    break;

                // Momentum
                case 2:
                    switch (number) {
                        case 2:   // U-Component of Wind [m s-1]
                        case 3:   // V-Component of Wind [m s-1]
                            return COLORMAPS['wind'];
                        case 8: return COLORMAPS['vertical-velocity'];   // Vertical Velocity (Pressure) [Pa s-1]
                        //           _ => None,
                        //       },
                    }
                    break;

                // Mass
                case 3:
                    switch (number) {
                        case 0: // Pressure [Pa]
                        case 1: // Pressure Reduced to MSL [Pa]
                            return COLORMAPS['pressure'];
                        case 5: return COLORMAPS['geopotential-height']; // Geopotential Height [gpm]
                    }
                    break;

                // Short-wave radiation
                case 4:
                    switch (number) {
                        case 7: return COLORMAPS['short-wave-radiation-flux'];
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


                // Miscellaneous
                case 191:
                    switch (number) {
                        case 192: return COLORMAPS['weather'];  // 天気
                    }
                    break;
            }
            break;

        // Oceanographic products
        case 10:
            switch (category) {

            }
            break;
    }


    return COLORMAPS['percentage']; // デフォルト
}

const MAX_COLORMAP_STEP = 100; // GLSL の for ループのインデックスは定数値しか比較できないので固定サイズにする。

const createGrayscaleColormap = (min, max, steps) => {
    const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
    const thresholds = new Float32Array(MAX_COLORMAP_STEP);
    let i = 0;
    for (let i = 0; i < steps; i++) {
        const d = i / (steps - 1);
        thresholds[i] = min + (max - min) * d; colors.set([d, d, d, 1.0], i * 4);
    }
    for (let i = steps; i < MAX_COLORMAP_STEP; i++) {
        thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
    }
    return { thresholds, colors };
}

const createColormaps = () => {
    {
        // temperature
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = -273.15; colors.set([0.0, 29 / 255, 114 / 255, 1.0], i * 4); i++;
        thresholds[i] = -5.0 + 273.15; colors.set([0.0, 57 / 255, 248 / 255, 1.0], i * 4); i++;
        thresholds[i] = 0.0 + 273.15; colors.set([0.0, 139 / 255, 250 / 255, 1.0], i * 4); i++;
        thresholds[i] = 5.0 + 273.15; colors.set([169 / 255, 232 / 255, 253 / 255, 1.0], i * 4); i++;
        thresholds[i] = 10.0 + 273.15; colors.set([255 / 255, 255 / 255, 239 / 255, 1.0], i * 4); i++;
        thresholds[i] = 15.0 + 273.15; colors.set([255 / 255, 255 / 255, 148 / 255, 1.0], i * 4); i++;
        thresholds[i] = 20.0 + 273.15; colors.set([252 / 255, 243 / 255, 55 / 255, 1.0], i * 4); i++;
        thresholds[i] = 25.0 + 273.15; colors.set([255 / 255, 143 / 255, 39 / 255, 1.0], i * 4); i++;
        thresholds[i] = 30.0 + 273.15; colors.set([255 / 255, 38 / 255, 27 / 255, 1.0], i * 4); i++;
        thresholds[i] = 35.0 + 273.15; colors.set([180 / 255, 8 / 255, 92 / 255, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([180 / 255, 8 / 255, 92 / 255, 1.0], i * 4);
        }



        colormaps['temperature'] = { thresholds, colors };
    }

    {
        // percentage
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([0.1, 0.1, 0.1, 1.0], i * 4); i++;
        thresholds[i] = 10.0; colors.set([0.2, 0.2, 0.2, 1.0], i * 4); i++;
        thresholds[i] = 20.0; colors.set([0.3, 0.3, 0.3, 1.0], i * 4); i++;
        thresholds[i] = 30.0; colors.set([0.4, 0.4, 0.4, 1.0], i * 4); i++;
        thresholds[i] = 40.0; colors.set([0.5, 0.5, 0.5, 1.0], i * 4); i++;
        thresholds[i] = 50.0; colors.set([0.6, 0.6, 0.6, 1.0], i * 4); i++;
        thresholds[i] = 60.0; colors.set([0.7, 0.7, 0.7, 1.0], i * 4); i++;
        thresholds[i] = 70.0; colors.set([0.8, 0.8, 0.8, 1.0], i * 4); i++;
        thresholds[i] = 80.0; colors.set([0.9, 0.9, 0.9, 1.0], i * 4); i++;
        thresholds[i] = 90.0; colors.set([1.0, 1.0, 1.0, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['percentage'] = { thresholds, colors };
    }

    {
        // cloud
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([0.0, 0.0, 0.0, 0.0], i * 4); i++;
        thresholds[i] = 10.0; colors.set([0.2, 0.2, 0.2, 1.0], i * 4); i++;
        thresholds[i] = 20.0; colors.set([0.3, 0.3, 0.3, 1.0], i * 4); i++;
        thresholds[i] = 30.0; colors.set([0.4, 0.4, 0.4, 1.0], i * 4); i++;
        thresholds[i] = 40.0; colors.set([0.5, 0.5, 0.5, 1.0], i * 4); i++;
        thresholds[i] = 50.0; colors.set([0.6, 0.6, 0.6, 1.0], i * 4); i++;
        thresholds[i] = 60.0; colors.set([0.7, 0.7, 0.7, 1.0], i * 4); i++;
        thresholds[i] = 70.0; colors.set([0.8, 0.8, 0.8, 1.0], i * 4); i++;
        thresholds[i] = 80.0; colors.set([0.9, 0.9, 0.9, 1.0], i * 4); i++;
        thresholds[i] = 90.0; colors.set([1.0, 1.0, 1.0, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['cloud'] = { thresholds, colors };
    }

    {
        // precipitation
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([0.0, 0.0, 0.0, 0.0], i * 4); i++;    // No echo
        thresholds[i] = 0.1; colors.set([240 / 255, 240 / 255, 254 / 255, 1.0], i * 4); i++;
        thresholds[i] = 1.0; colors.set([153 / 255, 204 / 255, 253 / 255, 1.0], i * 4); i++;
        thresholds[i] = 5.0; colors.set([44 / 255, 131 / 255, 251 / 255, 1.0], i * 4); i++;
        thresholds[i] = 10.0; colors.set([27 / 255, 65 / 255, 250 / 255, 1.0], i * 4); i++;
        thresholds[i] = 20.0; colors.set([253 / 255, 241 / 255, 49 / 255, 1.0], i * 4); i++;
        thresholds[i] = 30.0; colors.set([251 / 255, 143 / 255, 36 / 255, 1.0], i * 4); i++;
        thresholds[i] = 50.0; colors.set([250 / 255, 46 / 255, 28 / 255, 1.0], i * 4); i++;
        thresholds[i] = 80.0; colors.set([168 / 255, 23 / 255, 93 / 255, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['precipitation'] = { thresholds, colors };
    }

    {
        // total precipitation
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([0.0, 0.0, 0.0, 0.0], i * 4); i++;
        thresholds[i] = 0.1; colors.set([240 / 255, 240 / 255, 254 / 255, 1.0], i * 4); i++;
        thresholds[i] = 50.0; colors.set([153 / 255, 204 / 255, 253 / 255, 1.0], i * 4); i++;
        thresholds[i] = 80.0; colors.set([44 / 255, 131 / 255, 251 / 255, 1.0], i * 4); i++;
        thresholds[i] = 100.0; colors.set([27 / 255, 65 / 255, 250 / 255, 1.0], i * 4); i++;
        thresholds[i] = 150.0; colors.set([253 / 255, 241 / 255, 49 / 255, 1.0], i * 4); i++;
        thresholds[i] = 200.0; colors.set([251 / 255, 143 / 255, 36 / 255, 1.0], i * 4); i++;
        thresholds[i] = 250.0; colors.set([250 / 255, 46 / 255, 28 / 255, 1.0], i * 4); i++;
        thresholds[i] = 300.0; colors.set([168 / 255, 23 / 255, 93 / 255, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['total-precipitation'] = { thresholds, colors };
    }

    {
        // precipitation level
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([240 / 255, 240 / 255, 254 / 255, 1.0], i * 4); i++;
        thresholds[i] = 1.0; colors.set([153 / 255, 204 / 255, 253 / 255, 1.0], i * 4); i++;
        thresholds[i] = 5.0; colors.set([44 / 255, 131 / 255, 251 / 255, 1.0], i * 4); i++;
        thresholds[i] = 10.0; colors.set([27 / 255, 65 / 255, 250 / 255, 1.0], i * 4); i++;
        thresholds[i] = 20.0; colors.set([253 / 255, 241 / 255, 49 / 255, 1.0], i * 4); i++;
        thresholds[i] = 30.0; colors.set([251 / 255, 143 / 255, 36 / 255, 1.0], i * 4); i++;
        thresholds[i] = 50.0; colors.set([250 / 255, 46 / 255, 28 / 255, 1.0], i * 4); i++;
        thresholds[i] = 80.0; colors.set([168 / 255, 23 / 255, 93 / 255, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['precipitation-level'] = { thresholds, colors };
    }

    {
        // wind
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([240 / 255, 240 / 255, 254 / 255, 1.0], i * 4); i++;
        thresholds[i] = 5.0; colors.set([0 / 255, 57 / 255, 248 / 255, 1.0], i * 4); i++;
        thresholds[i] = 10.0; colors.set([252 / 255, 243 / 255, 55 / 255, 1.0], i * 4); i++;
        thresholds[i] = 15.0; colors.set([255 / 255, 143 / 255, 39 / 255, 1.0], i * 4); i++;
        thresholds[i] = 20.0; colors.set([255 / 255, 38 / 255, 27 / 255, 1.0], i * 4); i++;
        thresholds[i] = 25.0; colors.set([180 / 255, 8 / 255, 92 / 255, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['wind'] = { thresholds, colors };
    }

    {
        // weather
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        let i = 0;
        thresholds[i] = 0.0; colors.set([0 / 255, 0 / 255, 0 / 255, 1.0], i * 4); i++;
        thresholds[i] = 1.0; colors.set([255 / 255, 208 / 255, 148 / 255, 1.0], i * 4); i++;
        thresholds[i] = 2.0; colors.set([208 / 255, 208 / 255, 208 / 255, 1.0], i * 4); i++;
        thresholds[i] = 3.0; colors.set([128 / 255, 156 / 255, 252 / 255, 1.0], i * 4); i++;
        thresholds[i] = 4.0; colors.set([198 / 255, 229 / 255, 254 / 255, 1.0], i * 4); i++;
        thresholds[i] = 5.0; colors.set([247 / 255, 247 / 255, 255 / 255, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = Infinity; colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['weather'] = { thresholds, colors };
    }

    // short-wave radiation flux
    colormaps['short-wave-radiation-flux'] = createGrayscaleColormap(0.0, 1000.0, 10);

    // pressure
    colormaps['pressure'] = createGrayscaleColormap(95000.0, 105000.0, 10);

    // Vertical Velocity (Pressure)
    colormaps['vertical-velocity'] = createGrayscaleColormap(0.0, 1000.0, 10);

    // Geopotential Height
    colormaps['geopotential-height'] = createGrayscaleColormap(0.0, 1000.0, 10);

    {
        // sample
        const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
        const thresholds = new Float32Array(MAX_COLORMAP_STEP);
        for (let i = 0; i < MAX_COLORMAP_STEP; i++) {
            thresholds[i] = i; colors.set([i / (MAX_COLORMAP_STEP - 1), 0.0, i / (MAX_COLORMAP_STEP - 1, 1.0)], i * 4);
        }
        colormaps['sample'] = { thresholds, colors };
    }

    return colormaps;
}

const COLORMAPS = createColormaps();
