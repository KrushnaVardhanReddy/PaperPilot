export type ToastType = 'info' | 'success' | 'warning' | 'error';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration?: number;
}

export class ToastState {
  toasts = $state<Toast[]>([]);

  constructor() {}

  add(type: ToastType, message: string, duration: number = 3000) {
    const id = crypto.randomUUID();
    const toast: Toast = { id, type, message, duration };
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

  error(message: string, duration?: number) {
    this.add('error', message, duration);
  }
}

export const toastState = new ToastState();
