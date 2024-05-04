import { useQuery } from '@tanstack/react-query';
import { taskSvc } from '@/services/task';
export function useTaskData() {
  const { data: tasks } = useQuery({
    queryKey: ['tasks'],
    queryFn: taskSvc.getInCurrentDay
  });
  return { tasks };
}
