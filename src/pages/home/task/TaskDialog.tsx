import DatePickerWithInputField from '@/components/ui/datepicker';
import TimePickerWithInputField from '@/components/ui/timepicker';
import { useOutsideClick } from '@/hooks';
import { useListenEvent } from '@/hooks';
import { TaskData } from '@/interfaces';
import { BriefcaseBusiness, Clock, PencilLine, Trash } from 'lucide-react';
import moment from 'moment';
import { useEffect, useRef, useState } from 'react';

interface TaskDialogProps {
  open: boolean;
  onClose: () => void;
}

export function TaskDialog({ open, onClose }: TaskDialogProps) {
  const ref = useRef<HTMLDivElement>(null);
  useOutsideClick(ref, () => {
    onClose();
  });
  const [task, setTask] = useState<TaskData | null>(null);
  useListenEvent('dialog:open:mutate-task', (task: TaskData) => {
    setTask(task);
  });
  const dialogName = task ? 'Edit Task' : 'Add Task';
  useEffect(() => {
    console.log(task);
  }, [task]);

  return (
    open && (
      <div
        ref={ref}
        className='absolute -top-1/2 transform -translate-x-1/2 bg-white rounded-[15px] left-0 w-[424px] border border-[#D3D3D3] -translate-y-[100px] z-[99] overflow-y-auto no-scrollbar max-h-[500px] '
      >
        <div className='flex items-center justify-center w-full h-[65px] shadow-sm'>
          <span className='font-bold text-2xl leading-7 text-[#012F2F]'>{dialogName}</span>
        </div>
        <div className='flex flex-col items-center px-6 py-[18px] gap-y-[6px]'>
          <div className='relative w-full h-[50px]'>
            <input
              type='text'
              placeholder='Task Name'
              className='w-full h-full border border-[#D3D3D3] rounded-[5px] px-[14px] py-[10px] pl-[50px] placeholder:text-[#071A29] placeholder:font-[500] placeholder:text-md'
              value={task?.name}
              onChange={(e) => {
                setTask((prevTask: TaskData | null) => {
                  if (prevTask) {
                    return { ...prevTask, name: e.target.value };
                  }
                  return null;
                });
              }}
            />
            <span className='absolute transform -translate-y-1/2 left-3 top-1/2 w-7 h-7 px-[5px] py-[6px] bg-[#E9D8FC] rounded-[5px] flex justify-center items-center'>
              <PencilLine strokeWidth={2.75} size={16} />
            </span>
          </div>
          <div className='relative w-full h-[50px]'>
            <input
              type='text'
              placeholder='Project Name'
              className='w-full h-full border border-[#D3D3D3] rounded-[5px] px-[14px] py-[10px] pl-[50px] placeholder:text-[#071A29] placeholder:font-[500] placeholder:text-md'
            />
            <span className='absolute transform -translate-y-1/2 left-3 top-1/2 w-7 h-7 px-[5px] py-[6px] bg-[#D3FFD1] rounded-[5px] flex justify-center items-center'>
              <BriefcaseBusiness size={16} strokeWidth={2.75} />
            </span>
          </div>
          <DatePickerWithInputField
            value={task?.from ? moment.unix(task.from) : moment()}
            onChange={(val) => {
              console.log(val);
            }}
          />
          {/* <div className='flex items-start pl-[10px] pt-[10px] w-full justify-between '>
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
          </div> */}
          <div className='flex items-center justify-between w-full pl-[10px] pt-[10px]'>
            <div className='flex items-center'>
              <span className='w-7 h-7 px-[5px] py-[6px] bg-[#BDEBFB] rounded-[5px] flex justify-center items-center'>
                <Clock size={16} strokeWidth={2.75} />
              </span>
              <span className='text-[#071A29] font-[500] text-md ml-3'>Time</span>
            </div>
            <div className='flex items-center gap-x-2'>
              <div className='flex items-center gap-x-[8px]'>
                <TimePickerWithInputField
                  value={task?.from ? moment.unix(task.from) : null}
                  onChange={(val) => {
                    console.log(val);
                  }}
                />
                <span className='text-[#071A29] font-[500] text-md '>-</span>
                <TimePickerWithInputField
                  value={task?.to ? moment.unix(task.to) : null}
                  onChange={(val) => {
                    console.log(val);
                  }}
                />
              </div>
              <div
                onClick={() =>
                  setTask((prevTask: TaskData | null) => {
                    if (prevTask) {
                      return { ...prevTask, from: NaN, to: NaN };
                    }
                    return null;
                  })
                }
              >
                <Trash className='cursor-pointer ' size={16} strokeWidth={2.75} />
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
          >
            {' '}
            OK
          </button>
        </div>
      </div>
    )
  );
}
