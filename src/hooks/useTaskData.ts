import { taskSvc } from '@/services/task';
import { useQuery } from '@tanstack/react-query';

export function useTaskData() {
  const { data: tasks } = useQuery({
    queryKey: ['tasks'],
    queryFn: taskSvc.getInCurrentDay
  });
  return { tasks };
}
