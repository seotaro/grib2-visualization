'use strict';

const utils = require('./utils.js');

test("normalizeAngle のユニットテスト", () => {
  expect(utils.normalizeAngle(0.0)).toBe(0.0);
  expect(utils.normalizeAngle(180.0)).toBe(180.0);
  expect(utils.normalizeAngle(360.0)).toBe(0.0);
  expect(utils.normalizeAngle(720.0)).toBe(0.0);
  expect(utils.normalizeAngle(-180.0)).toBe(180.0);
  expect(utils.normalizeAngle(-360.0)).toBe(0.0);
  expect(utils.normalizeAngle(-720.0)).toBe(0.0);
});
