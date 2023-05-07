import React, { useState } from 'react';

import Box from '@mui/material/Box';
import Grid from '@mui/material/Grid';
import Slider from '@mui/material/Slider';
import TextField from '@mui/material/TextField';
import FormControl from '@mui/material/FormControl';
import FormControlLabel from '@mui/material/FormControlLabel';
import FormLabel from '@mui/material/FormLabel';
import Radio from '@mui/material/Radio';
import RadioGroup from '@mui/material/RadioGroup';
import Typography from '@mui/material/Typography';
import Checkbox from '@mui/material/Checkbox';

export const Settings = (props) => {
  const { initial,
    onChangeBlend,
    onChangeTextureFilter,
    onChangeViewMode,
    onChangeOpacity,
    onChangeCoastLine,
  } = props;

  return (<>
    <Box sx={{ m: 1 }}>
      {/* <Box sx={{}}>
        <TextureFilter initial={initial.textureFilter} onChange={onChangeTextureFilter} />
      </Box > */}
      <Box sx={{}}>
        <Grid container direction="row" justifyContent='space-between' alignItems="center" spacing={2}>
          <Grid item>
            <Blend initial={initial.blend} onChange={onChangeBlend} />
          </Grid >

          <Grid item>
            <ViewMode initial={initial.viewMode} onChange={onChangeViewMode} />
          </Grid >

          <Grid item>
            <CoastLine initial={initial.coastLine} onChange={(isShow) => onChangeCoastLine({ isShow })} />
          </Grid >
        </Grid >
      </Box>

      <Box sx={{}}>
        <Opacity initial={initial.opacity} onChange={onChangeOpacity} />
      </Box>
    </Box>
  </>)
}

const Opacity = ({ initial, onChange }) => {
  const toOpacityValue = (index) => {
    return index / 100.0;
  }

  const toOpacityIndex = (value) => {
    return Math.floor(value * 100.0);
  }

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
            <FormControlLabel value="normal" control={<Radio />} label="normal" />
            <FormControlLabel value="screen" control={<Radio />} label="screen" />
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
          <FormLabel id="texture-filter-radio-buttons-group-label">Texture Filter</FormLabel>
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
          <FormLabel id="view-mode-radio-buttons-group-label">View Mode</FormLabel>
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

// 海岸線
const CoastLine = ({ initial, onChange }) => {
  const [isShow, show] = useState(initial.isShow);

  const _onChange = (event) => {
    const flg = event.target.checked;
    show(flg);
    onChange(flg);
  };

  return (<>
    <FormControlLabel control={
      <Checkbox checked={isShow} onChange={_onChange} />
    } label="海岸線" />
  </>)
}
