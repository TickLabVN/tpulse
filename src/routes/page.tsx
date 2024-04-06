import { createFileRoute } from '@tanstack/react-router';
import moment from 'moment';
import { Box } from '@primer/react';
import {
  createColumnHelper,
  useReactTable,
  getCoreRowModel,
  RowModel,
  flexRender
} from '@tanstack/react-table';
import { useMemo } from 'react';

export const Route = createFileRoute('/page')({
  component: TimeTable
});

interface TimelineTable {
  hour: string;
  activity: string;
  planning: string;
}

function TimeTable() {
  const HOURS = Array.from({ length: 24 }, (_, i) =>
    moment().utcOffset('GMT+7').startOf('day').add(i, 'hours').format('HH:mm')
  );

  const columnHelper = createColumnHelper<TimelineTable>();
  const columnDefs = useMemo(
    () => [
      columnHelper.accessor('hour', {
        header: 'GMT +7',
        cell: (info) => <span className='flex items-start'>{info.getValue()}</span>
      }),
      columnHelper.accessor('activity', {
        header: () => (
          <Box
            sx={{
              padding: 2,
              fontWeight: 'bold'
            }}
          >
            Activity
          </Box>
        ),
        cell: () => (
          <Box
            sx={{
              height: '60px'
            }}
          ></Box>
        )
      }),
      columnHelper.accessor('planning', {
        header: () => (
          <Box
            sx={{
              padding: 2,
              fontWeight: 'bold'
            }}
          >
            Planning
          </Box>
        ),
        cell: () => (
          <Box
            sx={{
              height: '60px'
            }}
          ></Box>
        )
      })
    ],
    [columnHelper]
  );

  const timelineTable = useReactTable<TimelineTable>({
    columns: columnDefs,
    data: HOURS.map((hour) => ({
      hour,
      activity: '',
      planning: ''
    })),
    getCoreRowModel: getCoreRowModel<RowModel<TimelineTable>>()
  });

  return (
    <table className='w-full min-w-max table-auto text-left'>
      <thead>
        {timelineTable.getHeaderGroups().map((headerGroup) => (
          <tr key={headerGroup.id}>
            {headerGroup.headers.map((header, index) => (
              <th
                key={header.id}
                colSpan={header.colSpan}
                className={index > 0 ? 'bg-gray-200 border-b border-r border-gray-300' : ''}
              >
                {header.isPlaceholder
                  ? null
                  : flexRender(header.column.columnDef.header, header.getContext()) ?? ''}
              </th>
            ))}
          </tr>
        ))}
      </thead>
      <tbody>
        {timelineTable.getRowModel().rows.map((row) => (
          <tr key={row.id}>
            {row.getAllCells().map((cell, index) => (
              <td key={cell.id} className={'border-r border-gray-300' + (index > 0 && ' border-b')}>
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
