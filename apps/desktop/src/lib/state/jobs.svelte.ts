export type JobStatus = 'success' | 'error';

export interface Job {
  id: string;
  toolName: string;
  timestamp: number;
  status: JobStatus;
  message?: string;
}

export class JobsState {
  jobs = $state<Job[]>([]);

  constructor() {}

  addJob(toolName: string, status: JobStatus, message?: string) {
    const id = crypto.randomUUID();
    const timestamp = Date.now();
    this.jobs.push({ id, toolName, timestamp, status, message });
  }
}

export const jobsState = new JobsState();
