import { useEffect } from 'react';
export function useOutsideClick(ref: React.RefObject<HTMLElement>, onClose: () => void) {
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      const target = event.target as Node;
      const datePickerClass = 'MuiPickersSlideTransition-root';
      const timePickerClass = 'MuiPickersLayout-root';
      if (
        ref.current &&
        !ref.current.contains(target) &&
        !(target as HTMLElement).closest(`.${datePickerClass}`) &&
        !(target as HTMLElement).closest(`.${timePickerClass}`)
      ) {
        onClose();
      }
    }

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [ref, onClose]);
}
