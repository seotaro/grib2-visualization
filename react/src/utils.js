
// LineLayer の緯線経線データを返す。
export const latlonlineGeoJson = (() => {
    const d = 1;  // [°]。
    const dlon = 10;  // [°]。
    const dlat = 10;  // [°]。

    const geojson = {
        type: "FeatureCollection",
        features: [],
    };

    // 経線
    for (let lon = 180; -180 < lon; lon -= dlon) {
        const coordinates = [];
        for (let lat = -80; lat <= 80; lat += d) {
            coordinates.push([lon, lat]);
        }

        const feature = {
            type: "Feature",
            id: geojson.features.length,
            geometry: { type: 'LineString', coordinates: coordinates },
            properties: {},
            info: `${Math.abs(lon)}°${(lon < 0) ? 'W' : 'E'}`
        };
        geojson.features.push(feature);
    }

    // 緯線
    for (let lat = -80; lat < 90; lat += dlat) {
        const coordinates = [];
        for (let lon = -180; lon <= 180; lon += d) {
            coordinates.push([lon, lat]);
        }

        const feature = {
            type: "Feature",
            id: geojson.features.length,
            geometry: { type: 'LineString', coordinates: coordinates },
            properties: {},
            info: `${Math.abs(lat)}°${(lat < 0) ? 'S' : 'N'}`
        };
        geojson.features.push(feature);
    }

    return geojson;
})();




const MAX_COLORMAP_STEP = 100; // GLSL の for ループのインデックスは定数値しか比較できないので固定サイズにする。

const COLORMAPS = {};

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
    COLORMAPS['temperature'] = colormap;
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
    COLORMAPS['percentage'] = colormap;
}

export const colormaps = (category, number) => {
    switch (category) {
        case 0:
            {
                switch (number) {
                    case 0: return COLORMAPS['temperature'];
                }
            }
            break;
        //       1 => match number {
        //           1 => Some(String::from("Relative Humidity [%]")),
        //           8 => Some(String::from("Total Precipitation [kg m-2]")),
        //           203 => Some(String::from("Rime Factor []")),
        //           214 => Some(String::from(
        //               "Shallow Convective Moistening Rate [kg kg-1 s-1]",
        //           )),
        //           _ => None,
        //       },
        //       2 => match number {
        //           2 => Some(String::from("U-Component of Wind [m s-1]")),
        //           3 => Some(String::from("V-Component of Wind [m s-1]")),
        //           8 => Some(String::from("Vertical Velocity (Pressure) [Pa s-1]")),
        //           _ => None,
        //       },
        //       3 => match number {
        //           0 => Some(String::from("Pressure [Pa]")),
        //           1 => Some(String::from("Pressure Reduced to MSL [Pa]")),
        //           5 => Some(String::from("Geopotential Height [gpm]")),
        //           _ => None,
        //       },
        case 6:
            {
                switch (number) {
                    case 1:
                    case 3:
                    case 4:
                    case 5:
                        return COLORMAPS['percentage'];
                }
            }
        //       _ => None,
    }

    return COLORMAPS['percentage']; // デフォルト
}
