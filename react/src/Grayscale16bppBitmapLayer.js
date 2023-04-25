import { BitmapLayer } from '@deck.gl/layers';

import fragmentShader from './grayscale16bpp-bitmaplayer-fragment';

// Grayscale 16bpp のビットマップにレベル補正とガンマ補正を行い表示する。
// lower: シャドウ 0〜65535
// upper: ハイライト 0〜65535
// gamma: ガンマ 
export default class Grayscale16bppBitmapLayer extends BitmapLayer {
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
          colormap,
        })
        .draw();
    }
  }
}
