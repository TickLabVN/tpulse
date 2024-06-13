import DatePickerWithInputField from '@/components/ui/datepicker';
import TimePickerWithInputField from '@/components/ui/timepicker';
import { FeedPlusIcon } from '@primer/octicons-react';
import { BriefcaseBusiness, Clock, PencilLine, Trash } from 'lucide-react';
import { useEffect, useMemo, useRef, useState } from 'react';
import { TaskData } from '@/interfaces';
import { useTaskStore } from '@/states';
import { useOutsideClick } from '@/hooks';
import moment, { Moment } from 'moment';
import { InputItem } from './InputItem';
interface TaskDialogProps {
  open: boolean;
  onClose: () => void;
  taskData: TaskData | null;
}

export function TaskDialog({ open, onClose, taskData }: TaskDialogProps) {
  const ref = useRef<HTMLDivElement>(null);
  const [timeCount, setTimeCount] = useState(1);
  useOutsideClick(ref, () => {
    onClose();
    setTimeCount(1);
  });
  const { taskList, addTask, updateTask } = useTaskStore();
  const [newTaskList, setNewTaskList] = useState<TaskData[]>([]);
  const [task, setTask] = useState<TaskData>({
    id: taskData?.id ?? (Math.floor(Math.random() * 10000) + 1).toString(),
    taskName: taskData?.taskName ?? '',
    projectName: taskData?.projectName ?? '',
    date: taskData?.date ?? moment().unix(),
    start: taskData?.start ?? NaN,
    end: taskData?.end ?? NaN,
    color: taskData?.color ?? { backgroundColor: '#D8E6FC', textColor: '#071A29', borderColor: '#7BAFFDFA' }
  });
  useEffect(() => {
    setTask({
      id: taskData?.id ?? (Math.floor(Math.random() * 10000) + 1).toString(),
      taskName: taskData?.taskName ?? '',
      projectName: taskData?.projectName ?? '',
      date: taskData?.date ?? moment().unix(),
      start: taskData?.start ?? NaN,
      end: taskData?.end ?? NaN,
      color: taskData?.color ?? { backgroundColor: '#D8E6FC', textColor: '#071A29', borderColor: '#7BAFFDFA' }
    });
  }, [taskData]);
  const filteredTaskData = useMemo(() => {
    return taskList.filter(
      (task) =>
        task.projectName === taskData?.projectName &&
        task.taskName === taskData?.taskName &&
        task.date === taskData?.date
    );
  }, [taskList, taskData]);
  const handleAddTask = () => {
    if (newTaskList.length !== 0) {
      newTaskList.forEach((task) => {
        addTask({
          id: task.id,
          taskName: task.taskName,
          projectName: task.projectName,
          date: task.date,
          start: task.start,
          end: task.end,
          color: task.color
        });
      });
    } else {
      updateTask({
        id: task.id,
        taskName: task.taskName,
        projectName: task.projectName,
        date: task.date,
        start: task.start,
        end: task.end,
        color: task.color
      });
    }
    setNewTaskList([]);
    setTimeCount(1);
    onClose();
  };
  const handleAddTime = () => {
    setTimeCount((prev) => prev + 1);
    if (newTaskList.length !== 0) {
      setTask((task) => ({ ...task, id: (Math.floor(Math.random() * 10000) + 1).toString() }));
    }
    setNewTaskList([...newTaskList, task]);
  };

  return (
    open && (
      <div
        ref={ref}
        className='absolute -top-1/2 transform -translate-x-1/2 bg-white rounded-[15px] left-0 w-[424px] border border-[#D3D3D3] -translate-y-1 z-[99] overflow-y-auto no-scrollbar max-h-[500px] '
      >
        <div className='flex items-center justify-center w-full h-[65px] shadow-sm'>
          <span className='font-bold text-2xl leading-7 text-[#012F2F]'>
            {taskData ? 'Edit Task' : 'Add Task'}
          </span>
        </div>
        <div className='flex flex-col items-center px-6 py-[18px] gap-y-[6px]'>
          <div className='relative w-full h-[50px]'>
            <input
              type='text'
              placeholder='Task Name'
              className='w-full h-full border border-[#D3D3D3] rounded-[5px] px-[14px] py-[10px] pl-[50px] placeholder:text-[#071A29] placeholder:font-[500] placeholder:text-md'
              value={task.taskName}
              onChange={(e) => setTask({ ...task, taskName: e.target.value })}
            />
            <span className='absolute transform -translate-y-1/2 left-3 top-1/2 w-7 h-7 px-[5px] py-[6px] bg-[#E9D8FC] rounded-[5px] flex justify-center items-center'>
              <PencilLine strokeWidth={2.75} size={16} />
            </span>
          </div>
          <div className='relative w-full h-[50px]'>
            <input
              type='text'
              placeholder='Project Name'
              value={task.projectName}
              onChange={(e) => setTask({ ...task, projectName: e.target.value })}
              className='w-full h-full border border-[#D3D3D3] rounded-[5px] px-[14px] py-[10px] pl-[50px] placeholder:text-[#071A29] placeholder:font-[500] placeholder:text-md'
            />
            <span className='absolute transform -translate-y-1/2 left-3 top-1/2 w-7 h-7 px-[5px] py-[6px] bg-[#D3FFD1] rounded-[5px] flex justify-center items-center'>
              <BriefcaseBusiness size={16} strokeWidth={2.75} />
            </span>
          </div>
          <DatePickerWithInputField
            value={moment.unix(taskData ? taskData.date : moment().unix())}
            onChange={(newValue: Moment | null) => {
              if (newValue) {
                setTask({ ...task, date: newValue.unix() });
              }
            }}
          />
          <div className='flex items-start pl-[10px] pt-[10px] w-full justify-between '>
            <div className='flex items-center'>
              <span className='w-7 h-7 px-[5px] py-[6px] bg-[#BDEBFB] rounded-[5px] flex justify-center items-center'>
                <Clock size={16} strokeWidth={2.75} />
              </span>
              <span className='text-[#071A29] font-[500] text-md ml-3'>Time</span>
            </div>
            <div className='w-3/4 p-3 border rounded-[5px] border-[#D3D3D3] flex flex-col gap-y-[12px]'>
              {filteredTaskData.map((task, index) => (
                <InputItem key={task.id} task={task} index={index} setTask={setTask} />
              ))}
              {Array.from({ length: timeCount }).map((_, index) => (
                <div key={index} className='flex items-center justify-between'>
                  <div className='flex items-center gap-x-[8px]'>
                    <div className='w-[22px] h-[22px] bg-[#000000] px-[7px] text-white rounded-[5px]'>
                      {index + 1}
                    </div>
                    <TimePickerWithInputField
                      value={null}
                      onChange={(value: Moment | null) => {
                        if (value) {
                          setTask({ ...task, start: value.unix() });
                        }
                      }}
                    />
                    <span className='text-[#071A29] font-[500] text-md '>-</span>
                    <TimePickerWithInputField
                      value={null}
                      onChange={(value) => {
                        if (value) {
                          setTask({ ...task, end: value.unix() });
                        }
                      }}
                    />
                  </div>
                  <div
                    onClick={() => {
                      const newList: TaskData[] = [...newTaskList];
                      newList.splice(index, 1);
                      setNewTaskList(newList);
                      setTimeCount((prev) => prev - 1 || 1);
                    }}
                  >
                    <Trash className='cursor-pointer ' size={16} strokeWidth={2.75} />
                  </div>
                </div>
              ))}
              <div
                className='w-3/5 flex items-center p-2 border gap-[10px] border-dashed cursor-pointer border-[#D3D3D3] rounded-[5px]'
                onClick={handleAddTime}
              >
                <FeedPlusIcon size={16} className='stroke-1 text-green' />
                <span className='text-[#071A29] font-[500] text-md'>Add Time</span>
              </div>
            </div>
          </div>
        </div>
        <div className='flex items-center justify-center w-full h-[65px] border-t border-t-[#D3D3D3] gap-x-2'>
          <button
            className='rounded-lg py-2 px-4 border-[#D3D3D3] hover:bg-[#D3D3D3] hover:text-white active:bg-opacity-80'
            type='button'
            onClick={onClose}
          >
            {' '}
            Cancel{' '}
          </button>
          <button
            className='rounded-lg py-2 px-4 bg-[#1890FF] text-white hover:bg-[#40A9FF] active:bg-opacity-80'
            type='button'
            onClick={() => handleAddTask()}
          >
            {' '}
            OK
          </button>
        </div>
      </div>
    )
  );
}
