import React, { useState, useEffect } from 'react';

import DeckGL from '@deck.gl/react';
import { BitmapLayer, GeoJsonLayer, SolidPolygonLayer } from '@deck.gl/layers';
import { COORDINATE_SYSTEM, MapView, _GlobeView as GlobeView } from '@deck.gl/core';
import GL from '@luma.gl/constants';
import { Texture2D } from '@luma.gl/webgl'
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Slider from '@mui/material/Slider';
import TextField from '@mui/material/TextField';
import SettingsIcon from '@mui/icons-material/Settings';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import CloseIcon from '@mui/icons-material/Close';
import ViewListIcon from '@mui/icons-material/ViewList';
import FormControl from '@mui/material/FormControl';
import FormControlLabel from '@mui/material/FormControlLabel';
import FormLabel from '@mui/material/FormLabel';
import Radio from '@mui/material/Radio';
import RadioGroup from '@mui/material/RadioGroup';

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
const createGrayscale16bppTexture = (gl, pixels, width, height, filter) => {
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
    parameters: { ...textureParameters(filter) },
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
  const [blend, setBlend] = useState('normal');
  const [textureFilter, setTextureFilter] = useState('nearest');
  const [viewMode, setViewMode] = useState('globe');
  const [opacity, setOpacity] = useState(1.0);

  // 画面更新
  const [update, setUpdate] = useState(0);
  const redraw = () => {
    setUpdate((c) => c + 1);
  }

  useEffect(() => {
    (async () => {
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

        setImage(null);
        setItemIndex(0)
        grib2.clear();

        grib2.load(byteArray);
        setItems(grib2.items());
      });

      const grib2 = new wasm.Grib2Wrapper();
      setGrib2(grib2);
    }
  }, [rustWasm]);

  useEffect(() => {
    if ((gl != null) && rustWasm) {
      setImage(null);
      setTexture(null);
      if (0 < grib2.items().length) {
        const image = grib2.unpack_image(itemIndex);

        console.log('image', image
          , image.packing_type()
          , image.simple_packing_attributes()
          , image.run_length_packing_attributes());


        setImage(image);
      }
    }
  }, [itemIndex, items]);

  useEffect(() => {
    if ((gl != null) && image) {
      switch (image.packing_type()) {
        case 'simple':
          {
            const attributes = image.simple_packing_attributes();
            setTexture(createGrayscale16bppTexture(gl, attributes.pixels(), attributes.width, attributes.height, textureFilter));
          }
          break;

        case 'run-length':
          {
            const attributes = image.run_length_packing_attributes();
            setTexture(createGrayscale8bppTexture(gl, attributes.pixels(), attributes.width, attributes.height, textureFilter));
          }
          break;
      }
    }
  }, [image, textureFilter]);

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
    setViewMode(mode);
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
    const item = items[itemIndex];
    const colormap = colormaps(item.parameter_category, item.parameter_number);

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
          const bounds = attributes.bounds();
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
          layers.push(
            new RunLengthPackingBitmapLayer({
              id: "run-length-packing-bitmap-layer",
              getPolygonOffset: ({ layerIndex }) => [0, -layerIndex * 1000],
              bounds: [bounds.left, bounds.bottom, bounds.right, bounds.top].map(x => x / 1000000),
              _imageCoordinateSystem: COORDINATE_SYSTEM.LNGLAT,
              image: texture,
              opacity,
              factor: attributes.factor,
              levels: attributes.levels(),
              colormap,
              parameters,
            }),
          );
        }
        break;
    }
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
        <Box sx={{ position: 'relative', width: '50%', }}>
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
              : <MapView id="map" controller={true} />
            }
          </DeckGL>
        </Box>

        <Box sx={{ width: '50%', bgcolor: '#ffffff', }}
        >
          <Box sx={{ m: 1 }} >
            <Typography variant='h4' gutterBottom>
              GRIB2 Viewer
            </Typography>
            <input type='file' id='file-input' accept='.bin' />

            <Blend initial={blend} onChange={onChangeBlend} />
            <TextureFilter initial={textureFilter} onChange={onChangeTextureFilter} />
            <ViewMode initial={viewMode} onChange={onChangeViewMode} />
            <Opacity initial={opacity} onChange={onChangeOpacity} />
          </Box>

          <Grib2List
            initial={{ items, selection: [itemIndex] }}
            onChangeSelection={onChangeSelection}
          />
        </Box>
      </Box >
    </>
  );
}

const toOpacityValue = (index) => {
  return index / 100.0;
}

const toOpacityIndex = (value) => {
  return Math.floor(value * 100.0);
}

const Opacity = ({ initial, onChange }) => {
  const [opacityIndex, setOpacityIndex] = useState(toOpacityIndex(initial));  // 0<= opacityIndex <= 100

  const labelFormat = (index) => {
    return `${toOpacityValue(index)}`;
  }

  const _onChange = (event, newValue) => {
    const index = newValue;
    setOpacityIndex(index);
    onChange(toOpacityValue(index));
  };

  return (<>
    <Box sx={{ margin: 1, marginTop: 2 }}>

      <Grid container spacing={2} alignItems="center">
        <Grid item xs='auto'>
          <Typography id="opacity-input-slider" variant="subtitle1"  >
            Opacity
          </Typography>

        </Grid>
        <Grid item xs={7}>
          <Box sx={{ marginLeft: 1, marginRight: 1 }}>
            <Slider
              value={opacityIndex}
              onChange={_onChange}
              valueLabelDisplay="auto"
              min={0}
              max={100}
              valueLabelFormat={labelFormat}
              aria-labelledby="opacity-input-slider"
            />
          </Box>
        </Grid>

        <Grid item xs='auto'>
          <TextField
            id="opacity-input"
            value={toOpacityValue(opacityIndex)}
            variant="standard"
            InputProps={{
              readOnly: true,
              inputProps: {
                style: {
                  width: 50,
                  paddingRight: 10,
                  textAlign: 'right',
                  backgroundColor: 'lightgray',
                },
              }
            }}
          />
        </Grid>
      </Grid>
    </Box>
  </>);
}

const Blend = ({ initial, onChange }) => {
  const [type, setType] = useState(initial);

  const _onChange = (event) => {
    const type = event.target.value;
    setType(type);
    onChange(type);
  };

  return (<>
    <FormControl>
      <Grid container direction="row" justifyContent='flex-start' alignItems="center" spacing={1}>
        <Grid item>
          <FormLabel id="blend-radio-buttons-group-label">Blend</FormLabel>
        </Grid >

        <Grid item>
          <RadioGroup
            row
            aria-labelledby="blend-radio-buttons-group-label"
            name="blend-radio-buttons-group"
            value={type}
            onChange={_onChange}
          >
            <FormControlLabel value="normal" control={<Radio />} label="通常" />
            <FormControlLabel value="screen" control={<Radio />} label="スクリーン" />
          </RadioGroup>
        </Grid >
      </Grid >
    </FormControl>
  </>)
}

const TextureFilter = ({ initial, onChange }) => {
  const [type, setType] = useState(initial);

  const _onChange = (event) => {
    const type = event.target.value;
    setType(type);
    onChange(type);
  };

  return (<>
    <FormControl>
      <Grid container direction="row" justifyContent='flex-start' alignItems="center" spacing={1}>
        <Grid item>
          <FormLabel id="texture-filter-radio-buttons-group-label">TextureFilter</FormLabel>
        </Grid >

        <Grid item>
          <RadioGroup
            row
            aria-labelledby="texture-filter-radio-buttons-group-label"
            name="texture-filter-radio-buttons-group"
            value={type}
            onChange={_onChange}
          >
            <FormControlLabel value="nearest" control={<Radio />} label="nearest" />
            <FormControlLabel value="linear" control={<Radio />} label="linear" />
          </RadioGroup>
        </Grid >
      </Grid >
    </FormControl>
  </>)
}

const ViewMode = ({ initial, onChange }) => {
  const [type, setType] = useState(initial);

  const _onChange = (event) => {
    const type = event.target.value;
    setType(type);
    onChange(type);
  };

  return (<>
    <FormControl>
      <Grid container direction="row" justifyContent='flex-start' alignItems="center" spacing={1}>
        <Grid item>
          <FormLabel id="view-mode-radio-buttons-group-label">ViewMode</FormLabel>
        </Grid >

        <Grid item>
          <RadioGroup
            row
            aria-labelledby="view-mode-radio-buttons-group-label"
            name="view-mode-radio-buttons-group"
            value={type}
            onChange={_onChange}
          >
            <FormControlLabel value="globe" control={<Radio />} label="globe" />
            <FormControlLabel value="map" control={<Radio />} label="map" />
          </RadioGroup>
        </Grid >
      </Grid >
    </FormControl>
  </>)
}

export default App;
