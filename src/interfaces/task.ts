interface TaskColor {
  backgroundColor: string;
  textColor: string;
  borderColor: string;
}
interface TaskData {
  id: number;
  name: string;
  from: number;
  to: number;
  project_id: number | null;
  color: string;
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
interface TaskStore {
  taskList: TaskData[];
  selectedTask: TaskData | null;
  setSelectedTask: (task: TaskData | null) => void;
  addTask: (task: TaskData) => void;
  updateTask: (task: TaskData) => void;
  removeTask: (taskId: string) => void;
  syncTask: (date: string) => void;
}
export type { TaskData, TaskColor, TaskStore, SyncTaskData };
