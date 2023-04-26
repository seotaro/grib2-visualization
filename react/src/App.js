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
import { latlonlineGeoJson, colormaps } from './utils'
import SimplePackingBitmapLayer from './SimplePackingBitmapLayer'
import RunLengthPackingBitmapLayer from './RunLengthPackingBitmapLayer'
import init, * as wasm from './wasm/rust';

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

// Grayscale 8bpp のテクスチャを生成する
const createGrayscale8bppTexture = (gl, pixels, width, height) => {
  const texture = new Texture2D(gl, {
    data: pixels,
    format: GL.LUMINANCE,
    type: GL.UNSIGNED_BYTE,
    width,
    height,
    parameters: { ...DEFAULT_TEXTURE_PARAMETERS },
    pixelStore: { [GL.UNPACK_ALIGNMENT]: 1 },
    mipmaps: true,
  });

  return texture;
}

// Grayscale 16bpp のテクスチャを生成する
const createGrayscale16bppTexture = (gl, pixels, width, height) => {
  // Uint16Array から Uint8Array にキャストする。
  const dataView = new DataView(pixels.buffer);
  const dest = new Uint8Array(pixels.length * 2);
  for (let i = 0; i < pixels.length; i++) {
    const offset = i * 2;
    dest[offset + 0] = dataView.getUint8(offset + 0);
    dest[offset + 1] = dataView.getUint8(offset + 1);
  }

  const texture = new Texture2D(gl, {
    data: dest,
    format: GL.LUMINANCE_ALPHA,
    type: GL.UNSIGNED_BYTE,
    width,
    height,
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
  const [texture, setTexture] = useState(null);
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
        setImage(null);

        grib2.load(byteArray);
        setItems(grib2.items());
        setImage(grib2.unpack_image(itemIndex));
      });

      const grib2 = new wasm.Grib2Wrapper();
      setGrib2(grib2);
    }
  }, [rustWasm]);

  useEffect(() => {
    if ((gl != null) && rustWasm) {
      setImage(null);
      if (0 < grib2.items().length) {
        setImage(grib2.unpack_image(itemIndex));
      }
    }
  }, [itemIndex]);

  useEffect(() => {
    if ((gl != null)) {
      setTexture(null);
      if (image != null) {

        console.log('image', image
          , image.packing_type()
          , image.simple_packing_attributes()
          , image.run_length_packing_attributes());

        switch (image.packing_type()) {
          case 'simple':
            {
              const attributes = image.simple_packing_attributes();
              setTexture(createGrayscale16bppTexture(gl, attributes.pixels(), attributes.width, attributes.height));
            }
            break;

          case 'run-length':
            {
              const attributes = image.run_length_packing_attributes();
              setTexture(createGrayscale8bppTexture(gl, attributes.pixels(), attributes.width, attributes.height));
            }
            break;
        }
      }
    }
  }, [image]);

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
    const item = items[itemIndex];
    const colormap = colormaps(item.parameter_category, item.parameter_number);

    switch (image.packing_type()) {
      case 'simple':
        {
          const attributes = image.simple_packing_attributes();
          const bounds = attributes.bounds();
          layers.push(
            new SimplePackingBitmapLayer({
              id: "simple-packing-bitmap-layer",
              bounds: [bounds.left, bounds.bottom, bounds.right, bounds.top].map(x => x / 1000000),
              _imageCoordinateSystem: COORDINATE_SYSTEM.LNGLAT,
              image: texture,
              opacity: 0.75,
              r: attributes.r,
              e: attributes.e,
              d: attributes.d,
              colormap,
            }),
          );
        }
        break;

      case 'run-length':
        {
          const attributes = image.run_length_packing_attributes();
          const bounds = attributes.bounds();
          layers.push(
            new RunLengthPackingBitmapLayer({
              id: "run-length-packing-bitmap-layer",
              bounds: [bounds.left, bounds.bottom, bounds.right, bounds.top].map(x => x / 1000000),
              _imageCoordinateSystem: COORDINATE_SYSTEM.LNGLAT,
              image: texture,
              opacity: 0.75,
              factor: attributes.factor,
              levels: attributes.levels(),
              colormap,
            }),
          );
        }
        break;
    }
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
