import React, { useState, useEffect } from 'react';

import DeckGL from '@deck.gl/react';
import { BitmapLayer, GeoJsonLayer, SolidPolygonLayer } from '@deck.gl/layers';
import { COORDINATE_SYSTEM, MapView, _GlobeView as GlobeView } from '@deck.gl/core';
import GL from '@luma.gl/constants';
import { Texture2D } from '@luma.gl/webgl'
import Drawer from '@mui/material/Drawer';
import SettingsIcon from '@mui/icons-material/Settings';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import CloseIcon from '@mui/icons-material/Close';
import ViewListIcon from '@mui/icons-material/ViewList';

import { Grib2List } from './Components/Grib2List';
import { latlonlineGeoJson } from './utils'
import Grayscale16bppBitmapLayer from './Grayscale16bppBitmapLayer'
import init, * as wasm from './wasm/rust';

const MAX_COLORMAP = 100; // GLSL の for ループのインデックスは定数値しか比較できないので固定サイズにする。

const colormaps = [];
{
  const colormap = new Float32Array(MAX_COLORMAP * 4);
  for (let i = 0; i < MAX_COLORMAP; i++) {
    const threshold = i;
    const c = i / MAX_COLORMAP;
    colormap.set([threshold, c, c, c], i * 4);
  }
  colormaps.push(colormap);
}

const SETTINGS = {
  initialViewState: {
    longitude: 140.0,
    latitude: 40.0,
    zoom: 1.0,
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
  highlight: {
    color: [255, 127, 127, 127]
  },
};

const WIDTH = 256;
const HEIGHT = 256;

const DEFAULT_TEXTURE_PARAMETERS = {
  [GL.TEXTURE_MIN_FILTER]: GL.NEAREST,
  [GL.TEXTURE_MAG_FILTER]: GL.NEAREST,
  // [GL.TEXTURE_MIN_FILTER]: GL.LINEAR,
  // [GL.TEXTURE_MAG_FILTER]: GL.LINEAR,
  [GL.TEXTURE_WRAP_S]: GL.CLAMP_TO_EDGE,
  [GL.TEXTURE_WRAP_T]: GL.CLAMP_TO_EDGE,
};

// Grayscale 16bpp のテクスチャを生成する
const createGrayscale16bppTexture = (gl, image) => {
  const src = image.pixels();

  // Uint16Array から Uint8Array にキャストする。
  const dataView = new DataView(src.buffer);
  const dest = new Uint8Array(src.length * 2);
  for (let i = 0; i < src.length; i++) {
    const offset = i * 2;
    dest[offset + 0] = dataView.getUint8(offset + 0);
    dest[offset + 1] = dataView.getUint8(offset + 1);
  }

  const texture = new Texture2D(gl, {
    data: dest,
    format: GL.LUMINANCE_ALPHA,
    type: GL.UNSIGNED_BYTE,
    width: image.width,
    height: image.height,
    parameters: { ...DEFAULT_TEXTURE_PARAMETERS },
    pixelStore: { [GL.UNPACK_ALIGNMENT]: 2 },
    mipmaps: true,
  });

  return texture;
}

function App() {
  const [image, setImage] = useState(null);
  const [items, setItems] = useState(null);
  const [itemIndex, setItemIndex] = useState(0);
  const [grib2, setGrib2] = useState(null);
  const [gl, setGl] = useState(null);
  const [texture, setTexture] = useState({
    'grayscale16bpp': null
  });
  const [rustWasm, setWasm] = useState(null);
  const [isDrawerState, setDrawerState] = useState('close');

  useEffect(() => {
    (async () => {
      setDrawerState('close');  // ドロワーを初期状態にしたいので一旦、閉じる
      const rustWasm = await init();
      setWasm(rustWasm);
    })();
  }, []);

  useEffect(() => {
    if (rustWasm) {
      const fileInput = document.getElementById('file-input');
      fileInput.addEventListener('change', async (event) => {
        const file = event.target.files[0];
        const arrayBuffer = await file.arrayBuffer();
        const byteArray = new Uint8Array(arrayBuffer);

        grib2.clear();
        grib2.load(byteArray);

        const image = grib2.parse_simple_packing_image(itemIndex);
        setTexture({
          'grayscale16bpp': createGrayscale16bppTexture(gl, image)
        });
        setImage(image);

        const items = grib2.items();
        setItems(items);
      });

      const grib2 = new wasm.Grib2Wrapper();
      setGrib2(grib2);
    }
  }, [rustWasm]);

  useEffect(() => {
    if ((gl != null) && rustWasm) {
      setImage(null);
      if (0 < grib2.items().length) {
        const image = grib2.parse_simple_packing_image(itemIndex);
        setTexture({
          'grayscale16bpp': createGrayscale16bppTexture(gl, image)
        });
        setImage(image);
      }
    }
  }, [itemIndex]);

  const onChangeSelection = (selection) => {
    setItemIndex(selection);
  }

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
    layers.push(
      new Grayscale16bppBitmapLayer({
        id: "grayscale16bpp-bitmap-layer",
        bounds: [0.0, -90.0, 360.0, 90.0],
        _imageCoordinateSystem: COORDINATE_SYSTEM.LNGLAT,
        image: texture['grayscale16bpp'],
        opacity: 0.75,
        r: image ? image.r : 0.0,
        e: image ? image.e : 0,
        d: image ? image.d : 0,
        colormap: colormaps[0]
      }),
    );
  }

  layers.push(
    new GeoJsonLayer({
      id: "latlon-line-layer",
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
      <Box
        sx={{
          position: 'absolute',
          top: 10,
          left: 10,
          zIndex: 10,
        }}
      >
        <IconButton aria-label="list" size="large" color="primary" onClick={() => setDrawerState('open')}        >
          <ViewListIcon />
        </IconButton>

        <input type='file' id='file-input' />
      </Box>

      <DeckGL
        initialViewState={SETTINGS.initialViewState}
        controller={true}
        layers={layers}
        onWebGLInitialized={gl => {
          console.log(gl)
          setGl(gl);
        }}
      >
        <GlobeView id="map" width="100%" controller={true} resolution={1} />
        {/* <MapView id="map" width="100%" controller={true} /> */}
      </DeckGL>

      <Drawer
        anchor={'bottom'}
        open={isDrawerState === 'open'}
        onClick={(e) => setDrawerState('close')}
        onClose={(e) => setDrawerState('close')}
        hideBackdrop={true}
      >
        <Box
          sx={{ width: '100%', height: 300 }}
          onKeyDown={(event) => {
            if (event.type === 'keydown' && (event.key === 'Tab' || event.key === 'Shift')) {
              return;
            }
            setDrawerState('close');
          }}
          onClick={(e) => e.stopPropagation()}  // クリックでドロワーを閉じさせない
        >
          <IconButton aria-label="settings" size="large" color="primary"
            onClick={() => setDrawerState('close')}
          >
            <CloseIcon />
          </IconButton>

          <Grib2List
            initial={{ items, selection: [itemIndex] }}
            onChangeSelection={onChangeSelection}
          />
        </Box>
      </Drawer>
    </>
  );
}

export default App;
