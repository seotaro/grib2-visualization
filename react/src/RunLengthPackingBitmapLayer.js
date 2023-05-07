import { BitmapLayer } from '@deck.gl/layers';

import { MAX_COLORMAP_STEP } from './colormap-utils'
import fragmentShader from './run-length-packing-bitmaplayer-fragment';

// Run length packing データのレンダリング
// factor: Decimal scale factor
// levels : List of MVL 
export default class RunLengthPackingBitmapLayer extends BitmapLayer {
  getShaders() {
    const shaders = super.getShaders();
    shaders.fs = fragmentShader;
    return shaders;
  }

  draw(opts) {
    const { uniforms, moduleParameters } = opts;
    const { model, coordinateConversion, bounds, disablePicking } = this.state;
    const { image, desaturate, transparentColor, tintColor, factor, levels, colormap } = this.props;

    if (moduleParameters.pickingActive && disablePicking) {
      return;
    }

    const colors = new Float32Array(MAX_COLORMAP_STEP * 4);
    const thresholds = new Float32Array(MAX_COLORMAP_STEP);
    for (let i = 0; i < colormap.thresholds.length; i++) {
      thresholds[i] = colormap.thresholds[i];
    }
    for (let i = colormap.thresholds.length; i < MAX_COLORMAP_STEP; i++) {
      thresholds[i] = Infinity;
    }
    for (let i = 0; i < colormap.colors.length; i++) {
      colors.set(colormap.colors[i], i * 4);
    }
    for (let i = colormap.colors.length; i < MAX_COLORMAP_STEP; i++) {
      colors.set([1.0, 1.0, 1.0, 1.0], i * 4);
    }

    if (image && model) {
      model
        .setUniforms(uniforms)
        .setUniforms({
          bitmapTexture: image,
          desaturate,
          transparentColor: transparentColor.map(x => x / 255),
          tintColor: tintColor.slice(0, 3).map(x => x / 255),
          coordinateConversion,
          bounds,
          factor,
          levels,
          colors,
          thresholds,
        })
        .draw();
    }
  }
}
