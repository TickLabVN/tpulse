import { create } from 'zustand';
import { TaskData, TaskStore } from '@/interfaces';
import moment from 'moment';
import { syncTask } from '@/services';
import { SyncTaskData } from '@/interfaces';
export const useTaskStore = create<TaskStore>((set) => ({
  taskList: [
    {
      id: '1',
      taskName: 'Architecture Design',
      projectName: 'TPULSE',
      date: moment().startOf('month').add(27, 'days').unix(),
      start: moment().startOf('day').add(9, 'hours').unix(),
      end: moment().startOf('day').hours(10).minutes(30).unix(),
      color: {
        backgroundColor: '#E9D8FC',
        textColor: '#071A29',
        borderColor: '#907BFD'
      },
      status: 'done'
    },
    {
      id: '2',
      taskName: 'Developer Meeting',
      projectName: 'TPULSE',
      date: moment().startOf('month').add(27, 'days').unix(),
      start: moment().startOf('day').add(12, 'hours').unix(),
      end: moment().startOf('day').add(13, 'hours').unix(),
      color: {
        backgroundColor: '#D8E6FC',
        textColor: '#071A29',
        borderColor: '#7BAFFDFA'
      },
      status: 'todo'
    },
    {
      id: '3',
      taskName: 'Requirement Engineering',
      projectName: 'TPULSE',
      date: moment().startOf('month').add(27, 'days').unix(),
      start: moment().startOf('day').add(14, 'hours').unix(),
      end: moment().startOf('day').add(16, 'hours').unix(),
      color: {
        backgroundColor: '#FCF6D8',
        textColor: '#071A29',
        borderColor: '#FDC97BFA'
      },
      status: 'todo'
    },
    {
      id: '4',
      taskName: 'Requirement Engineering',
      projectName: 'TPULSE',
      date: moment().startOf('month').add(27, 'days').unix(),
      start: moment().startOf('day').add(20, 'hours').unix(),
      end: moment().startOf('day').add(21, 'hours').unix(),
      color: {
        backgroundColor: '#FCF6D8',
        textColor: '#071A29',
        borderColor: '#FDC97BFA'
      },
      status: 'todo'
    }
  ],
  selectedTask: null,
  setSelectedTask: (task) => set(() => ({ selectedTask: task })),
  addTask: (task) => set((state) => ({ taskList: [...state.taskList, task] })),
  updateTask: (task) =>
    set((state) => ({
      taskList: state.taskList.map((item) => (item.id === task.id ? task : item))
    })),
  removeTask: (taskId) => set((state) => ({ taskList: state.taskList.filter((task) => task.id !== taskId) })),
  syncTask: async (date) => {
    try {
      const syncTasks: SyncTaskData[] | undefined = await syncTask(date);
      if (syncTasks) {
        const tasks = syncTasks.map((task: SyncTaskData) => ({
          id: task.id,
          taskName: task.summary,
          projectName: 'TPULSE',
          date: moment(task.start.date).unix(),
          start: moment(task.start.date_time).unix(),
          end: moment(task.end.date_time).unix(),
          color: {
            backgroundColor: '#E9D8FC',
            textColor: '#071A29',
            borderColor: '#907BFD'
          }
        }));
        set((state) => {
          const newTasks = tasks.filter(
            (task) => !state.taskList.find((item: TaskData) => item.id === task.id)
          );
          return { taskList: [...state.taskList, ...newTasks] };
        });
      }
    } catch (error) {
      console.error(error);
    }
  }
}));
