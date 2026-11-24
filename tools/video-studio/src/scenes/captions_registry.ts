import { CaptionsMetadata } from './universal-scene';
import { captionsData_week01 } from './captions_week01';
import { captionsData_week02 } from './captions_week02';
import { captionsData_week03 } from './captions_week03';
import { captionsData_week04 } from './captions_week04';
import { captionsData_week05 } from './captions_week05';
import { captionsData_week06 } from './captions_week06';
import { captionsData_week07 } from './captions_week07';
import { captionsData_week08 } from './captions_week08';
import { captionsData_week09 } from './captions_week09';
import { captionsData_week10 } from './captions_week10';
import { captionsData_week11 } from './captions_week11';
import { captionsData_week12 } from './captions_week12';
import { captionsData_week13 } from './captions_week13';
import { captionsData_week14 } from './captions_week14';
import { captionsData_week15 } from './captions_week15';
import { captionsData_week16 } from './captions_week16';
import { captionsData_week17 } from './captions_week17';
import { captionsData_week18 } from './captions_week18';
import { captionsData_week19 } from './captions_week19';
import { captionsData_week20 } from './captions_week20';
import { captionsData_week21 } from './captions_week21';
import { captionsData_week22 } from './captions_week22';
import { captionsData_week23 } from './captions_week23';
import { captionsData_week24 } from './captions_week24';
import { captionsData_week25 } from './captions_week25';
import { captionsData_week26 } from './captions_week26';

export const CAPTIONS_MAP: Record<string, CaptionsMetadata> = {
  '01': captionsData_week01 as unknown as CaptionsMetadata,
  '02': captionsData_week02 as unknown as CaptionsMetadata,
  '03': captionsData_week03 as unknown as CaptionsMetadata,
  '04': captionsData_week04 as unknown as CaptionsMetadata,
  '05': captionsData_week05 as unknown as CaptionsMetadata,
  '06': captionsData_week06 as unknown as CaptionsMetadata,
  '07': captionsData_week07 as unknown as CaptionsMetadata,
  '08': captionsData_week08 as unknown as CaptionsMetadata,
  '09': captionsData_week09 as unknown as CaptionsMetadata,
  '10': captionsData_week10 as unknown as CaptionsMetadata,
  '11': captionsData_week11 as unknown as CaptionsMetadata,
  '12': captionsData_week12 as unknown as CaptionsMetadata,
  '13': captionsData_week13 as unknown as CaptionsMetadata,
  '14': captionsData_week14 as unknown as CaptionsMetadata,
  '15': captionsData_week15 as unknown as CaptionsMetadata,
  '16': captionsData_week16 as unknown as CaptionsMetadata,
  '17': captionsData_week17 as unknown as CaptionsMetadata,
  '18': captionsData_week18 as unknown as CaptionsMetadata,
  '19': captionsData_week19 as unknown as CaptionsMetadata,
  '20': captionsData_week20 as unknown as CaptionsMetadata,
  '21': captionsData_week21 as unknown as CaptionsMetadata,
  '22': captionsData_week22 as unknown as CaptionsMetadata,
  '23': captionsData_week23 as unknown as CaptionsMetadata,
  '24': captionsData_week24 as unknown as CaptionsMetadata,
  '25': captionsData_week25 as unknown as CaptionsMetadata,
  '26': captionsData_week26 as unknown as CaptionsMetadata,
};

export function getCaptionsForWeek(weekStr: string): CaptionsMetadata {
  return CAPTIONS_MAP[weekStr] || CAPTIONS_MAP['01'];
}
