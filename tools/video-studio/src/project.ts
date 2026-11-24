import { makeProject } from '@revideo/core';
import week01Scene from './scenes/week01-scene?scene';

export default makeProject({
  scenes: [week01Scene],
  settings: {
    shared: {
      size: { x: 1920, y: 1080 }, // 16:9 Landscape Full HD
      background: '#070a12',
    },
    rendering: {
      fps: 30,
    },
  },
});
