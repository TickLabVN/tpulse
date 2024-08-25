import { Text, Tooltip } from '@fluentui/react-components';

function truncateText(text: string, maxLength = 50) {
  return text.length > maxLength ? `${text.slice(0, maxLength)}...` : text;
}

type OverflowTextProps = {
  content: string;
  maxLength?: number;
  className?: string;
};

export function OverflowText({ content, className, maxLength }: OverflowTextProps) {
  return (
    <Tooltip content={content} relationship='label'>
      <Text className={`${className}`}>{truncateText(content, maxLength)}</Text>
    </Tooltip>
  );
}
