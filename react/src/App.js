import React, { useState, useEffect, useRef } from 'react';

import DeckGL from '@deck.gl/react';
import { BitmapLayer, GeoJsonLayer, SolidPolygonLayer } from '@deck.gl/layers';
import { COORDINATE_SYSTEM, MapView, _GlobeView as GlobeView } from '@deck.gl/core';
import GL from '@luma.gl/constants';
import { Texture2D } from '@luma.gl/webgl'
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Dropzone from 'react-dropzone'

import { Grib2List } from './Components/Grib2List';
import { Settings } from './Components/Settings';
import { latlonlineGeoJson, normalizeAngle } from './utils'
import {
  colormaps
  , createGrayscaleColormap
  , createRainbowColormap
  , normalizeRange
} from './colormap-utils'
import SimplePackingBitmapLayer from './SimplePackingBitmapLayer'
import RunLengthPackingBitmapLayer from './RunLengthPackingBitmapLayer'
import init, * as wasm from './wasm/rust';

const SETTINGS = {
  initialViewState: {
    longitude: 140.0,
    latitude: 40.0,
    zoom: 1.5,
  },
  mapLayer: {
    color: [64, 64, 64],
    url: 'https://d2ad6b4ur7yvpq.cloudfront.net/naturalearth-3.3.0/ne_50m_land.geojson'
  },
  backgroundLayer: {
    color: [32, 32, 32]
  },
  latlonLineLayer: {
    color: [127, 127, 127]
  },
  latlonGridLayer: {
    color: [127, 255, 127]
  },
  coastlineLayer: {
    color: [64, 64, 64],
    url: 'https://storage.googleapis.com/himawari-map-dataset/ne_10m_coastline.geojson'
  },
  highlight: {
    color: [255, 127, 127, 127]
  },
};


const textureParameters = (filter) => {
  const parameters = {
    [GL.TEXTURE_WRAP_S]: GL.CLAMP_TO_EDGE,
    [GL.TEXTURE_WRAP_T]: GL.CLAMP_TO_EDGE,
  };

  switch (filter) {
    case 'linear':
      parameters[GL.TEXTURE_MIN_FILTER] = GL.LINEAR;
      parameters[GL.TEXTURE_MAG_FILTER] = GL.LINEAR;
      break;

    case 'nearest':
    default:
      parameters[GL.TEXTURE_MIN_FILTER] = GL.NEAREST;
      parameters[GL.TEXTURE_MAG_FILTER] = GL.NEAREST;
      break;
  }

  return parameters;
};

// Grayscale 8bpp のテクスチャを生成する
const createGrayscale8bppTexture = (gl, pixels, width, height, filter) => {
  const texture = new Texture2D(gl, {
    data: pixels,
    format: GL.LUMINANCE,
    type: GL.UNSIGNED_BYTE,
    width,
    height,
    parameters: { ...textureParameters(filter) },
    pixelStore: { [GL.UNPACK_ALIGNMENT]: 1 },
    mipmaps: true,
  });

  return texture;
}

// Grayscale 16bpp のテクスチャを生成する
const createGrayscale16bppTexture = (gl, pixels, width, height, filter, isWrap) => {
  const dataView = new DataView(pixels.buffer);

  // Uint16Array から Uint8Array にキャストする。
  let dest = null;
  if (isWrap) {
    // 経度方向に一列分追加して経度方向にラップする
    dest = new Uint8Array((width + 1) * height * 2);

    for (let j = 0; j < height; j++) {
      for (let i = 0; i < width; i++) {
        const srcOffset = (width * j + i) * 2;
        const destOffset = ((width + 1) * j + i) * 2;
        dest[destOffset + 0] = dataView.getUint8(srcOffset + 0);
        dest[destOffset + 1] = dataView.getUint8(srcOffset + 1);
      }

      const srcOffset = (width * j + 0) * 2;
      const destOffset = ((width + 1) * j + width) * 2;
      dest[destOffset + 0] = dataView.getUint8(srcOffset + 0);
      dest[destOffset + 1] = dataView.getUint8(srcOffset + 1);
    }

    width++;

  } else {
    dest = new Uint8Array(pixels.length * 2);
    for (let i = 0; i < pixels.length; i++) {
      const offset = i * 2;
      dest[offset + 0] = dataView.getUint8(offset + 0);
      dest[offset + 1] = dataView.getUint8(offset + 1);
    }
  }

  const texture = new Texture2D(gl, {
    data: dest,
    format: GL.LUMINANCE_ALPHA,
    type: GL.UNSIGNED_BYTE,
    width,
    height,
    parameters: { ...textureParameters(filter) },
    pixelStore: { [GL.UNPACK_ALIGNMENT]: 2 },
    mipmaps: true,
  });

  return texture;
}

function App() {
  const [gl, setGl] = useState(null);
  const [grib2, setGrib2] = useState(null);
  const [image, setImage] = useState(null);
  const [items, setItems] = useState(null);
  const [itemIndex, setItemIndex] = useState(0);
  const [colormap, setColormap] = useState(null);
  const [texture, setTexture] = useState(null);
  const [files, setFiles] = useState([]);
  const [blend, setBlend] = useState('normal');
  const [textureFilter, setTextureFilter] = useState('nearest');
  const [viewMode, setViewMode] = useState('globe');
  const [opacity, setOpacity] = useState(1.0);
  const [isShowCoastLine, showCoastLine] = useState(true);

  const grib2ListRef = useRef();

  // 画面更新
  const [update, setUpdate] = useState(0);
  const redraw = () => {
    setUpdate((c) => c + 1);
  }

  useEffect(() => {
    (async () => {
      const rustWasm = await init();
      const grib2 = new wasm.Grib2Wrapper();
      setGrib2(grib2);
    })();
  }, []);


  useEffect(() => {
    if (gl && grib2) {
      setImage(null);
      setTexture(null);
      if (0 < grib2.items().length) {
        const image = grib2.unpack_image(itemIndex);

        console.log('image', image
          , image.packing_type()
          , image.simple_packing_attributes()
          , image.run_length_packing_attributes());

        const item = items[itemIndex];

        let colormap = colormaps(item.discipline, item.parameter_category, item.parameter_number);
        switch (image.packing_type()) {
          case 'simple':
            {
              const attributes = image.simple_packing_attributes();

              // 経度方向のラップ
              const bounds = attributes.bounds();
              const isWrap = normalizeAngle(bounds.right + attributes.di) === bounds.left;

              const min = (attributes.r + attributes.min * Math.pow(2.0, attributes.e)) / Math.pow(10.0, attributes.d);
              const max = (attributes.r + attributes.max * Math.pow(2.0, attributes.e)) / Math.pow(10.0, attributes.d);
              if (colormap == null) {
                const range = normalizeRange({ min, max });
                colormap = createRainbowColormap(range.min, range.max, 20);
              }
              console.log('attributes r:', attributes.r
                , 'e:', attributes.e
                , 'd:', attributes.d
                , 'min:', attributes.min, '（', min, '）'
                , 'max:', attributes.max, '（', max, '）'
                , 'wrap?:', isWrap);

              setTexture(createGrayscale16bppTexture(gl, attributes.pixels(), attributes.width, attributes.height, textureFilter, isWrap));
            }
            break;

          case 'run-length':
            {
              const attributes = image.run_length_packing_attributes();

              // 0 は欠測
              const levels = attributes.levels();
              const min = levels[(0 < attributes.min) ? attributes.min - 1 : 0] / Math.pow(10.0, attributes.factor);
              const max = levels[(0 < attributes.max) ? attributes.max - 1 : 0] / Math.pow(10.0, attributes.factor);
              if (colormap == null) {
                const range = normalizeRange({ min, max });
                colormap = createRainbowColormap(range.min, range.max, 20);
              }
              console.log('attributes factor:', attributes.factor
                , 'min:', attributes.min, '（', min, '）'
                , 'max:', attributes.max, '（', max, '）'
                , 'levels', levels);

              setTexture(createGrayscale8bppTexture(gl, attributes.pixels(), attributes.width, attributes.height, textureFilter));
            }
            break;
        }

        setColormap(colormap);
        setImage(image);
      }
    }
  }, [itemIndex, items, viewMode, textureFilter]);

  const onChangeSelection = (selection) => {
    setItemIndex(selection);
  }

  const onChangeOpacity = (opacity) => {
    setOpacity(opacity);
  };

  const onChangeBlend = (blend) => {
    setBlend(blend);
  };

  const onChangeTextureFilter = (filter) => {
    setTextureFilter(filter);
  };

  const onChangeViewMode = (mode) => {
    // テクスチャ作り直さないと壊れるっぽい
    setImage(null);
    setTexture(null);

    setViewMode(mode);
  };

  const onDropFiles = async (acceptedFiles) => {
    if (acceptedFiles == null) return;

    setItems(null);
    setImage(null);
    setTexture(null);
    setItemIndex(0);

    grib2.clear();

    setFiles(acceptedFiles.map(file => file.name));

    for (const file of acceptedFiles) {
      const arrayBuffer = await file.arrayBuffer();
      const byteArray = new Uint8Array(arrayBuffer);
      grib2.load(byteArray);
    };
    // grib2.dump();

    setItems(grib2.items());

    if (grib2ListRef.current) {
      grib2ListRef.current.initialize();
    }
  }

  const onChangeCoastLine = ({ isShow }) => {
    if (isShow != null) showCoastLine(isShow);
  };

  const layers = [];
  layers.push([
    new SolidPolygonLayer({
      id: 'background-layer',
      data: [[[-180, 90], [0, 90], [180, 90], [180, -90], [0, -90], [-180, -90]]],
      getPolygon: d => d,
      filled: true,
      getFillColor: SETTINGS.backgroundLayer.color,
    }),
    new GeoJsonLayer({
      id: "map-layer",
      data: SETTINGS.mapLayer.url,
      filled: true,
      getFillColor: SETTINGS.mapLayer.color,
    }),
  ]);

  if (image) {
    const parameters = ((blend) => {
      switch (blend) {
        case 'screen':
          return {
            [GL.BLEND]: true,
            [GL.BLEND_SRC_RGB]: GL.ONE,
            [GL.BLEND_DST_RGB]: GL.ONE_MINUS_SRC_COLOR,
            [GL.BLEND_SRC_ALPHA]: GL.ONE,
            [GL.BLEND_DST_ALPHA]: GL.ONE_MINUS_SRC_ALPHA,
          }

        case 'normal':
        default:
          return {
            [GL.BLEND]: true,
            [GL.BLEND_SRC_RGB]: GL.SRC_ALPHA,
            [GL.BLEND_DST_RGB]: GL.ONE_MINUS_SRC_ALPHA,
            [GL.BLEND_SRC_ALPHA]: GL.ONE,
            [GL.BLEND_DST_ALPHA]: GL.ONE_MINUS_SRC_ALPHA,
          }
      }
    })(blend);

    switch (image.packing_type()) {
      case 'simple':
        {
          const attributes = image.simple_packing_attributes();
          let bounds = attributes.bounds();

          // 経度方向のラップ
          if (normalizeAngle(bounds.right + attributes.di) === bounds.left) {
            bounds.right = bounds.right + attributes.di;
          }

          layers.push(
            new SimplePackingBitmapLayer({
              id: "simple-packing-bitmap-layer",
              getPolygonOffset: ({ layerIndex }) => [0, -layerIndex * 1000],
              bounds: [bounds.left, bounds.bottom, bounds.right, bounds.top].map(x => x / 1000000),
              _imageCoordinateSystem: COORDINATE_SYSTEM.LNGLAT,
              image: texture,
              opacity,
              r: attributes.r,
              e: attributes.e,
              d: attributes.d,
              colormap,
              parameters,
            }),
          );
        }
        break;

      case 'run-length':
        {
          const attributes = image.run_length_packing_attributes();
          const bounds = attributes.bounds();

          // シェーダーに渡す配列は固定サイズ
          const buffer = new ArrayBuffer(256 * 2);
          const levels = new Int16Array(buffer);
          levels.fill(0, 0, 256);
          levels.set(attributes.levels(), 0);

          layers.push(
            new RunLengthPackingBitmapLayer({
              id: "run-length-packing-bitmap-layer",
              getPolygonOffset: ({ layerIndex }) => [0, -layerIndex * 1000],
              bounds: [bounds.left, bounds.bottom, bounds.right, bounds.top].map(x => x / 1000000),
              _imageCoordinateSystem: COORDINATE_SYSTEM.LNGLAT,
              image: texture,
              opacity,
              factor: attributes.factor,
              levels: levels,
              colormap,
              parameters,
            }),
          );
        }
        break;
    }
  }

  if (isShowCoastLine) {
    layers.push(
      new GeoJsonLayer({
        id: "coastlineLayer-layer",
        getPolygonOffset: ({ layerIndex }) => [0, -layerIndex * 1000],
        data: SETTINGS.coastlineLayer.url,
        stroked: true,
        getLineColor: SETTINGS.coastlineLayer.color,
        lineWidthUnits: 'pixels',
        lineWidthScale: 1,
        getLineWidth: 1,
      }),
    )
  }

  layers.push(
    new GeoJsonLayer({
      id: "latlon-line-layer",
      getPolygonOffset: ({ layerIndex }) => [0, -layerIndex * 1000],
      data: latlonlineGeoJson,
      stroked: true,
      getLineColor: SETTINGS.latlonLineLayer.color,
      lineWidthUnits: 'pixels',
      lineWidthScale: 1,
      getLineWidth: 1,
    }),
  );

  return (
    <>
      <Box sx={{
        position: 'absolute',
        top: 0,
        left: 0,
        bottom: 0,
        right: 0,

        display: 'flex',
        flexDirection: 'row',
      }}>
        <Box sx={{ position: 'relative', width: '50%' }}>
          <DeckGL
            initialViewState={SETTINGS.initialViewState}
            controller={true}
            layers={layers}
            onWebGLInitialized={gl => {
              console.log(gl)
              setGl(gl);
            }}
          >
            {(viewMode === 'globe')
              ? <GlobeView id="globe" controller={true} resolution={1} />
              : <MapView id="map" controller={true} repeat={true} />
            }
          </DeckGL>

          <Box sx={{
            position: 'absolute', bottom: 20, width: '100%',
            display: 'flex', flexDirection: 'row', justifyContent: 'center',
          }}>
            <Contour
              colormap={colormap}
              unit={(items && items[itemIndex]) ? items[itemIndex].parameter_unit : null}
              isLevelValue={image?.packing_type() === 'run-length'}
            />
          </Box >
        </Box>

        <Box sx={{ position: 'relative', width: '50%', bgcolor: '#ffffff', overflow: 'auto' }}
        >
          <Box sx={{ m: 1 }} >
            <Typography variant='h4' gutterBottom>
              GRIB2 Viewer
            </Typography>

            <Dropzone onDrop={onDropFiles} accept={{ 'application/octet-stream': ['.bin'] }}>
              {({ getRootProps, getInputProps }) => (
                <Box sx={{ m: 1 }}>
                  <Box sx={{
                    p: 1,
                    bgcolor: '#fafafa',
                    color: 'darkgray',
                    borderRadius: 2,
                    borderWidth: 2,
                    borderStyle: 'dashed',
                    borderColor: 'lightgray',
                  }}
                    {...getRootProps()}
                  >
                    <input {...getInputProps()} />
                    <Box sx={{ m: 0.5 }}>
                      Drop GRIB2（*.bin） files here, or click to select files
                    </Box>

                    <Box sx={{ height: '4em', overflowY: 'scroll' }} >
                      <ol>{files.map(name => <li key={name}>{name}</li>)}</ol>
                    </Box>
                  </Box>
                </Box>
              )}
            </Dropzone>
            <Settings
              initial={{
                blend,
                textureFilter,
                viewMode,
                opacity,
                coastLine: { isShow: isShowCoastLine },
              }}
              onChangeBlend={onChangeBlend}
              onChangeTextureFilter={onChangeTextureFilter}
              onChangeViewMode={onChangeViewMode}
              onChangeOpacity={onChangeOpacity}
              onChangeCoastLine={onChangeCoastLine}
            />

            <Grib2List
              ref={grib2ListRef}
              initial={{ items, selection: itemIndex }}
              onChangeSelection={onChangeSelection}
            />
            <Information item={(items && items[itemIndex]) ? items[itemIndex] : null} />
          </Box>
        </Box>
      </Box >
    </>
  );
}

const Information = ({ item }) => {
  return (<Box sx={{
    m: 1,
    p: 1,
    minHeight: 100,
    color: 'gray',
    borderRadius: 1,
    borderWidth: 1,
    borderStyle: 'solid',
    borderColor: 'lightgray',
    overflow: 'auto',
  }}>
    <pre>
      {item?.information
        ? item.information
        : 'None of information'
      }
    </pre>
  </Box>)
}

const Contour = ({ colormap, unit, isLevelValue }) => {
  if (colormap == null) return <></>

  const width = 400 / colormap.colors.length;
  const height = 10;

  const normalizeColor = (rgba) => {
    const r = Math.ceil(rgba[0] * 255);
    const g = Math.ceil(rgba[1] * 255);
    const b = Math.ceil(rgba[2] * 255);
    return `${r}, ${g}, ${b}, 1.0`;
  }

  // レベル値の場合、カラーマップの最初のステップは欠測を表すので次のステップから表示する。 

  return <>
    <Box sx={{ display: 'flex', flexDirection: 'row', alignItems: 'baseline' }}>
      <Box sx={{ height, mr: 1, color: 'darkgray', }} >
        {colormap.thresholds[isLevelValue ? 1 : 0]}
      </Box>

      <Box sx={{ display: 'flex', flexDirection: 'row', border: 'solid thin darkgray' }}>
        {colormap.colors
          .filter((_, i) => !(isLevelValue && (i === 0)))
          .map((x, i) => {
            return <Box key={i} sx={{ width, height, bgcolor: `rgba(${normalizeColor(x)})` }} />
          })}
      </Box>

      <Box sx={{ height, ml: 1, color: 'darkgray', }} >
        {colormap.thresholds[colormap.thresholds.length - 1]}
      </Box>

      {unit &&
        <Box sx={{ height, ml: 1, color: 'darkgray', }} >
          {`[${unit}]`}
        </Box>
      }
    </Box>
  </>
}

export default App;
