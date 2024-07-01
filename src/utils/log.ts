import { type LogOptions, debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

type LogFn = (message: string, options?: LogOptions) => Promise<void>;

const createLogFn = (fn: LogFn) => {
  return (data: unknown, message?: string) => {
    const msg: Record<string, unknown> = {};
    if (message) msg.message = message;
    if (typeof data === 'object') Object.assign(msg, data);
    else msg.data = data;

    fn(JSON.stringify(msg));
  };
};

export const log = {
  info: createLogFn(info),
  error: createLogFn(error),
  warn: createLogFn(warn),
  debug: createLogFn(debug),
  trace: createLogFn(trace)
};
