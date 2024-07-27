import { TextField } from '@mui/material';
import type { UseTimeFieldProps } from '@mui/x-date-pickers/TimeField';
import { TimePicker, type TimePickerProps } from '@mui/x-date-pickers/TimePicker';
import type {
  BaseSingleInputFieldProps,
  FieldSection,
  TimeValidationError
} from '@mui/x-date-pickers/models';
import moment, { type Moment } from 'moment';
import * as React from 'react';

interface ButtonFieldProps
  extends UseTimeFieldProps<Moment, false>,
    BaseSingleInputFieldProps<Moment | null, Moment, FieldSection, false, TimeValidationError> {
  setOpen?: React.Dispatch<React.SetStateAction<boolean>>;
}

function InputField(props: ButtonFieldProps) {
  const {
    setOpen,
    label,
    id,
    disabled,
    InputProps: { ref } = {},
    inputProps: { 'aria-label': ariaLabel } = {}
  } = props;
  return (
    <TextField
      placeholder='hh:mm'
      id={id}
      disabled={disabled}
      ref={ref}
      aria-label={ariaLabel}
      onClick={() => setOpen?.((prev) => !prev)}
      value={label}
      className='w-[80px] border border-[#D3D3D3] rounded-[5px] px-[8px] py-[1px] placeholder:text-[#071A29] placeholder:font-[400] placeholder:text-md'
      sx={{
        '& .MuiInputBase-input': {
          padding: '1px 8px 1px 8px',
          height: '28px'
        }
      }}
    />
  );
}
function InputTimePicker(props: Omit<TimePickerProps<Moment>, 'open' | 'onOpen' | 'onClose'>) {
  const [open, setOpen] = React.useState(false);
  return (
    <TimePicker
      slots={{ ...props.slots, field: InputField }}
      // biome-ignore lint/suspicious/noExplicitAny: <explanation>
      slotProps={{ ...props.slotProps, field: { setOpen } as any }}
      {...props}
      open={open}
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
    />
  );
}
interface TimePickerWithInputFieldProps {
  value: Moment | null;
  onChange: (newTime: Moment | null) => void;
}
export default function TimePickerWithInputField({ value, onChange }: TimePickerWithInputFieldProps) {
  const [time, setTime] = React.useState<Moment | null>(value);
  return (
    <InputTimePicker
      label={time ? moment(time).format('hh:mma') : ''}
      value={time}
      onChange={(newTime) => {
        setTime(newTime);
        onChange(newTime);
      }}
    />
  );
}
