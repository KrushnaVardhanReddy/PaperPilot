import { makeProject } from '@revideo/core';
import deepdiveData from './configs/tools_deepdive.json' with { type: 'json' };
import { createSingleToolScene, SingleToolData, ToolCaptionsMetadata } from './scenes/single-tool-scene';
import { allToolCaptions } from './scenes/all_tool_captions';
import { CURRENT_TOOL_NAME } from './current_tool_target';

const toolEntry = (deepdiveData as SingleToolData[]).find(t => t.tool_name === CURRENT_TOOL_NAME) || (deepdiveData as SingleToolData[])[0];
const captions = (allToolCaptions[CURRENT_TOOL_NAME] || allToolCaptions['pdf_compress']) as ToolCaptionsMetadata;

const scene = createSingleToolScene(toolEntry, captions);

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
