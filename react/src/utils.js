
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
