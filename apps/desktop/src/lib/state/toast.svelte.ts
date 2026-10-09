export type ToastType = 'info' | 'success' | 'warning' | 'error';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  actionHint?: string;
  technicalDetails?: string;
  duration?: number;
}

export class ToastState {
  toasts = $state<Toast[]>([]);

  constructor() {}

  add(type: ToastType, message: string, duration: number = 3000, actionHint?: string, technicalDetails?: string) {
    const id = crypto.randomUUID();
    const toast: Toast = { id, type, message, actionHint, technicalDetails, duration };
    this.toasts.push(toast);

    if (duration > 0) {
      setTimeout(() => {
        this.remove(id);
      }, duration);
    }
  }

  remove(id: string) {
    this.toasts = this.toasts.filter(t => t.id !== id);
  }

  info(message: string, duration?: number) {
    this.add('info', message, duration);
  }

  success(message: string, duration?: number) {
    this.add('success', message, duration);
  }

  warning(message: string, duration?: number) {
    this.add('warning', message, duration);
  }

  error(message: string, arg2?: string | number, arg3?: string, arg4?: number) {
    let duration = 4000;
    let actionHint: string | undefined;
    let technicalDetails: string | undefined;

    if (typeof arg2 === 'number') {
      duration = arg2;
    } else if (typeof arg2 === 'string') {
      actionHint = arg2;
      technicalDetails = arg3;
      if (typeof arg4 === 'number') {
        duration = arg4;
      }
    }

    this.add('error', message, duration, actionHint, technicalDetails);
  }
}

export const toastState = new ToastState();
