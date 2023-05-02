import { BitmapLayer } from '@deck.gl/layers';

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
          colors: colormap.colors,
          thresholds: colormap.thresholds,
        })
        .draw();
    }
  }
}
