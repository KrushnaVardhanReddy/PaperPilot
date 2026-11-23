import { makeProject } from '@revideo/core';
import week01Scene from './scenes/week01-scene?scene';

export default makeProject({
  scenes: [week01Scene],
  settings: {
    shared: {
      size: { x: 1080, y: 1920 }, // Vertical 9:16 HD for YouTube Shorts
      background: '#080c14',
    },
    rendering: {
      fps: 30,
    },
  },
});
