
export const colormaps = (category, number) => {
    switch (category) {
        case 0:
            switch (number) {
                case 0: return COLORMAPS['temperature'];    // "Temperature [K]"
            }
            break;

        case 1:
            switch (number) {
                case 1: return COLORMAPS['percentage']; // Relative Humidity [%]
                case 8: return COLORMAPS['total-precipitation'];    // Total Precipitation [kg m-2]
                case 201: return COLORMAPS['precipitation'];    // 10分間降水強度（１時間換算値）レベル値
                case 203: return COLORMAPS['precipitation'];    // 降水強度レベル値(解析、予報）
            }
            break;
        //       },
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

        case 6:
            switch (number) {
                case 1: // Total Cloud Cover [%]
                case 3: // Low Cloud Cover [%]
                case 4: // Medium Cloud Cover [%]
                case 5: // High Cloud Cover [%]
                    return COLORMAPS['percentage'];
            }
    }

    return COLORMAPS['percentage']; // デフォルト
}

const MAX_COLORMAP_STEP = 100; // GLSL の for ループのインデックスは定数値しか比較できないので固定サイズにする。

const createGrayscaleColormap = (min, max, steps) => {
    const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
    let i = 0;
    for (let i = 0; i < steps; i++) {
        const d = i / (steps - 1);
        colormap.set([min + (max - min) * d, d, d, d], i * 4);
    }
    for (let i = steps; i < MAX_COLORMAP_STEP; i++) {
        colormap.set([Infinity, 1.0, 1.0, 1.0], i * 4);
    }
    return colormap;
}

const createColormaps = () => {
    {
        // temperature
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        let i = 0;
        colormap.set([-273.15, 0.0, 29 / 255, 114 / 255], i * 4); i++;
        colormap.set([-5.0 + 273.15, 0.0, 57 / 255, 248 / 255], i * 4); i++;
        colormap.set([0.0 + 273.15, 0.0, 139 / 255, 250 / 255], i * 4); i++;
        colormap.set([5.0 + 273.15, 169 / 255, 232 / 255, 253 / 255], i * 4); i++;
        colormap.set([10.0 + 273.15, 255 / 255, 255 / 255, 239 / 255], i * 4); i++;
        colormap.set([15.0 + 273.15, 255 / 255, 255 / 255, 148 / 255], i * 4); i++;
        colormap.set([20.0 + 273.15, 252 / 255, 243 / 255, 55 / 255], i * 4); i++;
        colormap.set([25.0 + 273.15, 255 / 255, 143 / 255, 39 / 255], i * 4); i++;
        colormap.set([30.0 + 273.15, 255 / 255, 38 / 255, 27 / 255], i * 4); i++;
        colormap.set([35.0 + 273.15, 180 / 255, 8 / 255, 92 / 255], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([Infinity, 180 / 255, 8 / 255, 92 / 255], i * 4);
        }
        colormaps['temperature'] = colormap;
    }

    {
        // percentage
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        let i = 0;
        colormap.set([0.0, 0.1, 0.1, 0.1], i * 4); i++;
        colormap.set([10.0, 0.2, 0.2, 0.2], i * 4); i++;
        colormap.set([20.0, 0.3, 0.3, 0.3], i * 4); i++;
        colormap.set([30.0, 0.4, 0.4, 0.4], i * 4); i++;
        colormap.set([40.0, 0.5, 0.5, 0.5], i * 4); i++;
        colormap.set([50.0, 0.6, 0.6, 0.6], i * 4); i++;
        colormap.set([60.0, 0.7, 0.7, 0.7], i * 4); i++;
        colormap.set([70.0, 0.8, 0.8, 0.8], i * 4); i++;
        colormap.set([80.0, 0.9, 0.9, 0.9], i * 4); i++;
        colormap.set([90.0, 1.0, 1.0, 1.0], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([Infinity, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['percentage'] = colormap;
    }

    {
        // precipitation
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        let i = 0;
        colormap.set([0.0, 1.0, 0.0, 1.0], i * 4); i++;    // No echo
        colormap.set([0.1, 240 / 255, 240 / 255, 254 / 255], i * 4); i++;
        colormap.set([1.0, 153 / 255, 204 / 255, 253 / 255], i * 4); i++;
        colormap.set([5.0, 44 / 255, 131 / 255, 251 / 255], i * 4); i++;
        colormap.set([10.0, 27 / 255, 65 / 255, 250 / 255], i * 4); i++;
        colormap.set([20.0, 253 / 255, 241 / 255, 49 / 255], i * 4); i++;
        colormap.set([30.0, 251 / 255, 143 / 255, 36 / 255], i * 4); i++;
        colormap.set([50.0, 250 / 255, 46 / 255, 28 / 255], i * 4); i++;
        colormap.set([80.0, 168 / 255, 23 / 255, 93 / 255], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([Infinity, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['precipitation'] = colormap;
    }

    {
        // total precipitation
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        let i = 0;
        colormap.set([0.0, 0.0, 0.0, 0.0], i * 4); i++;
        colormap.set([0.1, 240 / 255, 240 / 255, 254 / 255], i * 4); i++;
        colormap.set([50.0, 153 / 255, 204 / 255, 253 / 255], i * 4); i++;
        colormap.set([80.0, 44 / 255, 131 / 255, 251 / 255], i * 4); i++;
        colormap.set([100.0, 27 / 255, 65 / 255, 250 / 255], i * 4); i++;
        colormap.set([150.0, 253 / 255, 241 / 255, 49 / 255], i * 4); i++;
        colormap.set([200.0, 251 / 255, 143 / 255, 36 / 255], i * 4); i++;
        colormap.set([250.0, 250 / 255, 46 / 255, 28 / 255], i * 4); i++;
        colormap.set([300.0, 168 / 255, 23 / 255, 93 / 255], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([Infinity, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['total-precipitation'] = colormap;
    }

    {
        // precipitation level
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        let i = 0;
        colormap.set([0.0, 240 / 255, 240 / 255, 254 / 255], i * 4); i++;
        colormap.set([1.0, 153 / 255, 204 / 255, 253 / 255], i * 4); i++;
        colormap.set([5.0, 44 / 255, 131 / 255, 251 / 255], i * 4); i++;
        colormap.set([10.0, 27 / 255, 65 / 255, 250 / 255], i * 4); i++;
        colormap.set([20.0, 253 / 255, 241 / 255, 49 / 255], i * 4); i++;
        colormap.set([30.0, 251 / 255, 143 / 255, 36 / 255], i * 4); i++;
        colormap.set([50.0, 250 / 255, 46 / 255, 28 / 255], i * 4); i++;
        colormap.set([80.0, 168 / 255, 23 / 255, 93 / 255], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([Infinity, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['precipitation-level'] = colormap;
    }

    {
        // wind
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        let i = 0;
        colormap.set([0.0, 240 / 255, 240 / 255, 254 / 255], i * 4); i++;
        colormap.set([5.0, 0 / 255, 57 / 255, 248 / 255], i * 4); i++;
        colormap.set([10.0, 252 / 255, 243 / 255, 55 / 255], i * 4); i++;
        colormap.set([15.0, 255 / 255, 143 / 255, 39 / 255], i * 4); i++;
        colormap.set([20.0, 255 / 255, 38 / 255, 27 / 255], i * 4); i++;
        colormap.set([25.0, 180 / 255, 8 / 255, 92 / 255], i * 4); i++;
        for (; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([Infinity, 1.0, 1.0, 1.0], i * 4);
        }
        colormaps['wind'] = colormap;
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
        const colormap = new Float32Array(MAX_COLORMAP_STEP * 4);
        for (let i = 0; i < MAX_COLORMAP_STEP; i++) {
            colormap.set([i, i / (MAX_COLORMAP_STEP - 1), 0.0, i / (MAX_COLORMAP_STEP - 1)], i * 4);
        }
        colormaps['sample'] = colormap;
    }

    return colormaps;
}

const COLORMAPS = createColormaps();
