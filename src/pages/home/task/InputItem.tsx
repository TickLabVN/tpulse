import { Trash } from 'lucide-react';
import TimePickerWithInputField from '@/components/ui/timepicker';
import { TaskData } from '@/interfaces';
import moment, { Moment } from 'moment';
import { useTaskStore } from '@/states';
interface InputItemProps {
  task: TaskData;
  index: number;
  setTask: (task: TaskData) => void;
}

export function InputItem({ task, index, setTask }: InputItemProps) {
  // console.log('task', task);
  const { removeTask } = useTaskStore();
  return (
    <div className='flex items-center justify-between'>
      <div className='flex items-center gap-x-[8px]'>
        <div className='w-[22px] h-[22px] bg-[#000000] px-[7px] text-white rounded-[5px]'>{index + 1}</div>
        <TimePickerWithInputField
          value={task.start ? moment.unix(task.start) : null}
          onChange={(value: Moment | null) => {
            if (value) {
              setTask({ ...task, start: value.unix() });
            }
          }}
        />
        <span className='text-[#071A29] font-[500] text-md '>-</span>
        <TimePickerWithInputField
          value={task.end ? moment.unix(task.end) : null}
          onChange={(value: Moment | null) => {
            if (value) {
              setTask({ ...task, end: value.unix() });
            }
          }}
        />
      </div>
      <div
        onClick={() => {
          removeTask(task.id);
        }}
      >
        <Trash className='cursor-pointer ' size={16} strokeWidth={2.75} />
      </div>
    </div>
  );
}
