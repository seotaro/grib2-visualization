import { BitmapLayer } from '@deck.gl/layers';

import fragmentShader from './simple-packing-bitmaplayer-fragment';

// Simple packing データのレンダリング
// R: Reference value
// E: Binary Scale Factor
// D: Decimal Scale Factor
export default class SimplePackingBitmapLayer extends BitmapLayer {
  getShaders() {
    const shaders = super.getShaders();
    shaders.fs = fragmentShader;
    return shaders;
  }

  draw(opts) {
    const { uniforms, moduleParameters } = opts;
    const { model, coordinateConversion, bounds, disablePicking } = this.state;
    const { image, desaturate, transparentColor, tintColor, r, e, d, colormap } = this.props;

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
          r, e, d,
          colors: colormap.colors,
          thresholds: colormap.thresholds,
        })
        .draw();
    }
  }
}
