export type JobStatus = 'running' | 'success' | 'error' | 'cancelled';

export interface Job {
  id: string;
  toolName: string;
  timestamp: number;
  status: JobStatus;
  message?: string;
  progress?: number;
  currentPage?: number;
  totalPages?: number;
}

export class JobsState {
  jobs = $state<Job[]>([]);

  constructor() {}

  addJob(toolName: string, status: JobStatus, message?: string) {
    const id = crypto.randomUUID();
    const timestamp = Date.now();
    this.jobs.push({ id, toolName, timestamp, status, message });
    return id; // Return ID so we can update it
  }

  updateJobProgress(id: string, progress: number, currentPage?: number, totalPages?: number) {
    const job = this.jobs.find(j => j.id === id);
    if (job) {
      job.progress = progress;
      if (currentPage !== undefined) job.currentPage = currentPage;
      if (totalPages !== undefined) job.totalPages = totalPages;
    }
  }

  updateJobStatus(id: string, status: JobStatus, message?: string) {
    const job = this.jobs.find(j => j.id === id);
    if (job) {
      job.status = status;
      if (message !== undefined) job.message = message;
    }
  }
}

export const jobsState = new JobsState();
