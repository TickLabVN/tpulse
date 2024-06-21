import TextField from '@mui/material/TextField';
import { UseDateFieldProps } from '@mui/x-date-pickers/DateField';
import { DatePicker, DatePickerProps } from '@mui/x-date-pickers/DatePicker';
import { BaseSingleInputFieldProps, DateValidationError, FieldSection } from '@mui/x-date-pickers/models';
import { CalendarDays } from 'lucide-react';
import moment, { Moment } from 'moment';
import * as React from 'react';

interface ButtonFieldProps
  extends UseDateFieldProps<Moment, false>,
    BaseSingleInputFieldProps<Moment | null, Moment, FieldSection, false, DateValidationError> {
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
    <div className='relative w-full h-[50px]'>
      <TextField
        id={id}
        disabled={disabled}
        ref={ref}
        aria-label={ariaLabel}
        onClick={() => setOpen?.((prev) => !prev)}
        value={label}
        className='border border-[#D3D3D3] rounded-[5px] placeholder:text-[#071A29] placeholder:font-[500] placeholder:text-md py-[10px]'
        sx={{
          '& .MuiInputBase-root': {
            padding: '10px 14px 10px 50px'
          },
          '& .MuiInputBase-input': {
            padding: '0',
            height: '28px',
            margin: '1px'
          }
        }}
        fullWidth={true}
      />
      <span className='absolute transform -translate-y-1/2 left-3 top-1/2 w-7 h-7 px-[5px] py-[6px] bg-[#D3FFD1] rounded-[5px] flex justify-center items-center'>
        <CalendarDays size={16} strokeWidth={2.75} />
      </span>
    </div>
  );
}

function InputDatePicker(props: Omit<DatePickerProps<Moment>, 'open' | 'onOpen' | 'onClose'>) {
  const [open, setOpen] = React.useState(false);

  return (
    <DatePicker
      slots={{ ...props.slots, field: InputField }}
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      slotProps={{ ...props.slotProps, field: { setOpen } as any }}
      {...props}
      open={open}
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
    />
  );
}
interface DatePickerWithInputFieldProps {
  value: Moment | null;
  onChange: (newValue: Moment | null) => void;
}
export default function DatePickerWithInputField({ value, onChange }: DatePickerWithInputFieldProps) {
  const [date, setDate] = React.useState<Moment | null>(value);
  return (
    <InputDatePicker
      label={date ? moment(date).format('ddd, DD/MM/YYYY') : ''}
      value={date}
      onChange={(newValue) => {
        setDate(newValue);
        onChange(newValue);
      }}
    />
  );
}
