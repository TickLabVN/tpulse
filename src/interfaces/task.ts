interface TaskColor {
  backgroundColor: string;
  textColor: string;
  borderColor: string;
}

interface SyncTaskData {
  id: string;
  summary: string;
  start: {
    date_time: string;
    date: string;
  };
  end: {
    date_time: string;
    date: string;
  };
}

export type { TaskColor, SyncTaskData };
