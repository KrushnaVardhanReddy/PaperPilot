import { makeProject } from '@revideo/core';
import { TOOLS_DATA } from './configs/tools-data';
import { createUniversalToolScene } from './scenes/universal-scene';
import { getCaptionsForWeek } from './scenes/captions_registry';
import { CURRENT_WEEK } from './current_target';

const toolConfig = TOOLS_DATA[CURRENT_WEEK] || TOOLS_DATA[1];
const captionsData = getCaptionsForWeek(toolConfig.weekStr);
const scene = createUniversalToolScene(toolConfig, captionsData);

export default makeProject({
  scenes: [scene],
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



