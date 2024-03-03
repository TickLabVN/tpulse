import { useState, useRef } from 'react';
import { TextInput, Text, Box, Button, Checkbox, Dialog, Select, ButtonGroup } from '@primer/react';
import { DiffAddedIcon, CalendarIcon, PinIcon, TagIcon } from '@primer/octicons-react';

export function DayView() {
  interface Item {
    title: string;
    start: string;
    end: string;
    color: string;
  }
  const [newTaskName, setNewTaskName] = useState<string>('');
  const [newTaskPriority, setNewTaskPriority] = useState<string>('');
  const [newTaskType, setNewTaskType] = useState<string>('');
  const [newStartTime, setNewStartTime] = useState<string>('');
  const [newEndTime, setNewEndTime] = useState<string>('');
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const now = new Date();
  const currentDate = now.toLocaleDateString();
  const getCurrentDayOfWeek = (): string => {
    const daysOfWeek = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
    const currentDate = new Date();
    const currentDayOfWeek = daysOfWeek[currentDate.getDay()];
    return currentDayOfWeek;
  };
  const handleDragStart = (handleName: string, hour: string) => (e: DragEvent) => {
    const index = hours.findIndex((elementHour) => elementHour === hour) + 1;
    const bottomElement = document.getElementById(hour);
    const topElement = document.getElementById(hours[index]);
    if (bottomElement && topElement) {
      const bottomPosition = bottomElement.getBoundingClientRect().top;
      const y = e?.clientY;
      const rangeItem = Math.round((convertTimeToNumber(hours[1]) - convertTimeToNumber(hours[0])) * 60);
      const subMinute =
        Math.round(parseInt(hours[index - 2].slice(3))) + Math.round((rangeItem / 60) * (y - bottomPosition));
      const exactMinute = subMinute >= 60 ? subMinute - 60 : subMinute;
      const exactHour =
        exactMinute < parseInt(hours[index - 2].slice(3))
          ? parseInt(hours[index - 2]) + 1
          : parseInt(hours[index - 2]);
      const exactHourToString = `${exactHour.toString()}:${exactMinute}`;
      if (handleName === 'AFK' || handleName === 'ACTIVITY') {
        setStartTime(exactHourToString);
      } else {
        setNewStartTime(exactHourToString);
      }
    } else {
      alert('Error');
    }
  };
  const handleDragEnd = (handleName: string) => (e: React.DragEvent) => {
    const endElement = document.elementFromPoint(e.clientX, e.clientY) as HTMLElement;
    const hour = endElement.id;
    const index = hours.findIndex((elementHour) => elementHour === hour) + 1;
    const bottomElement = document.getElementById(hour);
    const topElement = document.getElementById(hours[index]);
    if (bottomElement && topElement) {
      const bottomPosition = bottomElement.getBoundingClientRect().top;
      const y = e?.clientY;
      const rangeItem = Math.round((convertTimeToNumber(hours[1]) - convertTimeToNumber(hours[0])) * 60);
      const subMinute =
        Math.round(parseInt(hours[index - 2].slice(3))) + Math.round((rangeItem / 60) * (y - bottomPosition));
      const exactMinute = subMinute >= 60 ? subMinute - 60 : subMinute;
      const exactHour =
        exactMinute < parseInt(hours[index - 2].slice(3))
          ? parseInt(hours[index - 2]) + 1
          : parseInt(hours[index - 2]);
      const exactHourToString = `${exactHour.toString()}:${exactMinute}`;
      if (handleName === 'AFK' || handleName === 'ACTIVITY') {
        handleSubmitRange(startTime, exactHourToString);
      } else {
        setNewEndTime(exactHourToString);
        setIsOpen(true);
      }
    } else {
      alert('Error');
    }
  };
  const resetNewTask = () => {
    setNewTaskName('');
    setNewTaskPriority('');
    setNewEndTime('');
    setNewTaskType('');
    setNewStartTime('');
  };
  const handleAddTask = () => {
    const newTask = {
      id: planData.length + 1, // Tạo id mới cho task
      title: newTaskName,
      priority: newTaskPriority,
      type: newTaskType,
      start: newStartTime,
      end: newEndTime,
      color: 'red'
    };

    setPlanData([...planData, newTask]);
    resetNewTask();
    setIsOpen(false);
  };
  const placeItem = (item: Item) => {
    const eachTimeRange = convertTimeToNumber(hours[1]) - convertTimeToNumber(hours[0]);
    const start = Math.max(convertTimeToNumber(item.start), convertTimeToNumber(hours[0]));
    const end = Math.min(convertTimeToNumber(item.end), convertTimeToNumber(hours[hours.length - 1]));
    const range = end < start ? 24 - start + end : end - start;
    const height = (range / eachTimeRange) * 60;
    const top = ((start - convertTimeToNumber(hours[0])) / eachTimeRange) * 60;
    return {
      height: `${Math.min(height, (hours.length - 1) * 60)}px`,
      top: Number.isNaN(top) ? '100px' : `${100 + top}px`
    };
  };
  const filterItems = (item: Item) => {
    const startItemTime = convertTimeToNumber(item.start);
    const endItemTime =
      convertTimeToNumber(item.end) < startItemTime
        ? 24 + convertTimeToNumber(item.end)
        : convertTimeToNumber(item.end);
    const startHourTime = convertTimeToNumber(hours[0]);
    const endHourTime =
      convertTimeToNumber(hours[hours.length - 1]) < startHourTime
        ? 24
        : convertTimeToNumber(hours[hours.length - 1]);
    return (
      (startItemTime >= startHourTime && startItemTime < endHourTime) ||
      (endItemTime > startHourTime && endItemTime <= endHourTime) ||
      (startItemTime <= startHourTime && endItemTime >= endHourTime)
    );
  };
  const hour = Array.from({ length: 24 }, (_, i) => {
    const date = new Date();
    date.setHours(i + 7, 0, 0, 0);
    return date.toISOString().split('T')[1].substring(0, 5);
  });
  const [hours, setHours] = useState(hour);
  const items = [
    {
      title: 'Youtube',
      start: '13:00:00.000',
      end: '14:00:00.000',
      color: 'red'
    },
    {
      title: 'Facebook',
      start: '07:00:00.000',
      end: '12:00:00.000',
      color: 'blue'
    }
  ];
  const [planData, setPlanData] = useState([
    {
      id: 1,
      title: 'Conceptual design',
      priority: 'High',
      type: 'Work',
      start: '7:00:00.000',
      end: '8:00:00.000',
      color: 'red'
    },
    {
      id: 2,
      title: 'Requirement Specification',
      priority: 'Medium',
      type: 'Work',
      start: '12:00:00.000',
      end: '18:00:00.000',
      color: 'blue'
    },
    {
      id: 3,
      title: 'Media planning',
      priority: 'Medium',
      type: 'Work',
      start: '14:00:00.000',
      end: '16:00:00.000',
      color: 'yellow'
    }
  ]);
  const headers = [
    {
      name: 'AFK',
      width: '20px',
      items: items
    },
    {
      name: 'ACTIVITY',
      width: '100%',
      items: items
    },
    {
      name: 'PLAN',
      width: '100%',
      items: planData
    }
  ];
  // const [scale, setScale] = useState(1);
  // const [dragStartCell, setDragStartCell] = useState(null);
  // const [dragEndCell, setDragEndCell] = useState(null);
  const [startTime, setStartTime] = useState<string>('');
  const [endTime, setEndTime] = useState<string>('');
  const startRef = useRef<HTMLInputElement>(null);
  const endRef = useRef<HTMLInputElement>(null);
  const convertTimeToNumber = (time: string) => {
    const timeComponents = time.split(':').map((component) => parseInt(component, 10) || 0);
    return (
      timeComponents[0] +
      timeComponents[1] / 60 +
      (timeComponents[2] || 0) / 3600 +
      (timeComponents[3] || 0) / 3600000
    );
  };
  const convertNumberToTime = (currentHour: number) => {
    const formattedHour = Math.floor(currentHour).toString().padStart(2, '0');
    const formattedMinute = Math.floor((currentHour % 1) * 60)
      .toString()
      .padStart(2, '0');
    const formattedSecond = Math.floor(((currentHour * 60) % 1) * 60)
      .toString()
      .padStart(2, '0');
    const formattedMillisecond = Math.floor(((currentHour * 60 * 60) % 1) * 1000)
      .toString()
      .padStart(3, '0');
    let formattedTime = formattedHour + ':' + formattedMinute;
    formattedTime = formattedSecond === '000' ? formattedTime + ':' + formattedSecond : formattedTime;
    formattedTime =
      formattedMillisecond === '000' ? formattedTime : formattedTime + '.' + formattedMillisecond;
    return formattedTime;
  };
  const handleSubmitRange = (startInput: string, endInput: string) => {
    // Extract hours, minutes, and seconds from the input values
    const parsedStartTime = convertTimeToNumber(startInput);
    const parsedEndTime = convertTimeToNumber(endInput);
    if (isNaN(parsedStartTime) || isNaN(parsedEndTime) || parsedStartTime >= parsedEndTime) {
      // Handle invalid input or show an error message
      console.error('Invalid input. Please make sure the start time is before the end time.');
      return;
    }
    const stepSize = (parsedEndTime - parsedStartTime) / 12;
    const newHour = Array.from({ length: 13 }, (_, i) => {
      const currentHour = parsedStartTime + i * stepSize;
      return convertNumberToTime(currentHour);
    });
    setHours(newHour);
  };
  return (
    <Box
      sx={{
        padding: '50px'
      }}
    >
      <Text
        sx={{
          fontSize: '32px',
          fontWeight: 'bold',
          opacity: '70%'
        }}
      >
        {`${getCurrentDayOfWeek()} ${currentDate}`}
      </Text>
      <Box
        sx={{
          marginTop: 2,
          display: 'flex',
          columnGap: 4
        }}
      >
        <TextInput
          placeholder='hh:mm'
          ref={startRef}
          sx={{
            width: '200px',
            height: '40px',
            border: '1px solid #ccc'
          }}
          onChange={(e) => setStartTime(e.target.value)}
        />
        <TextInput
          placeholder='hh:mm'
          ref={endRef}
          sx={{
            width: '200px',
            height: '40px',
            border: '1px solid #ccc'
          }}
          onChange={(e) => setEndTime(e.target.value)}
        />
        <Button
          sx={{
            width: '200px',
            height: '40px',
            border: '1px solid #ccc',
            backgroundColor: 'rgb(254 226 226)'
          }}
          onClick={() => handleSubmitRange(startTime, endTime)}
        >
          Submit
        </Button>
        <Button
          sx={{
            width: '200px',
            height: '40px',
            border: '1px solid #ccc',
            backgroundColor: 'rgb(254 226 226)'
          }}
          onClick={() => setHours(hour)}
        >
          Reset
        </Button>
      </Box>
      <Box
        sx={{
          height: 'fit-content',
          overflowY: 'auto',
          padding: '50px',
          display: 'flex',
          gap: 8
        }}
      >
        <Box
          sx={{
            flex: 8,
            display: 'flex'
          }}
        >
          <Box
            sx={{
              width: '50px',
              flexShrink: 0
            }}
          >
            <Box
              sx={{
                height: '40px',
                textAlign: 'right',
                fontSize: '14px',
                position: 'relative',
                paddingLeft: '8px'
              }}
            >
              <Text
                sx={{
                  position: 'absolute',
                  bottom: 0,
                  transform: 'translateY(50%) translateX(-80px)',
                  userSelect: 'none'
                }}
              >
                GMT+7
              </Text>
            </Box>
            {hours.map((hour, index) => (
              <Box
                key={index}
                sx={{
                  height: '60px',
                  paddingLeft: '8px',
                  position: 'relative',
                  textAlign: 'right',
                  fontSize: '14px',
                  borderRight: '1px solid #ccc',
                  borderBottom: '1px solid transparent'
                }}
              >
                <Text
                  sx={{
                    position: 'absolute',
                    bottom: 0,
                    transform: 'translateY(50%) translateX(-80px)',
                    userSelect: 'none'
                  }}
                >
                  {hour}
                </Text>
              </Box>
            ))}
          </Box>
          <Box sx={{ flex: '1' }}>
            <Box
              sx={{
                display: 'grid',
                gridTemplateColumns: '5rem 1fr 1fr',
                justifyItems: 'center'
              }}
            >
              {headers.map((header, i) => (
                <Box
                  key={i}
                  sx={{
                    display: 'grid',
                    gridTemplateColumns: '1fr',
                    width: '100%',
                    textAlign: 'center',
                    position: 'relative'
                  }}
                >
                  <Box
                    sx={{
                      padding: 2,
                      borderBottom: '1px solid #ccc',
                      position: 'relative'
                    }}
                  >
                    {header.name}
                    {i === 0 && (
                      <Text
                        sx={{
                          position: 'absolute',
                          bottom: 0,
                          left: 0,
                          transform: 'translateX(-75%) translateY(1px)',
                          width: '20px',
                          height: '1px',
                          backgroundColor: '#ccc'
                        }}
                      ></Text>
                    )}
                    {i === headers.length - 1 && (
                      <Text
                        sx={{
                          position: 'absolute',
                          bottom: 0,
                          right: 0,
                          width: '1px',
                          height: '20px',
                          backgroundColor: '#ccc'
                        }}
                      />
                    )}
                    <Text
                      sx={{
                        position: 'absolute',
                        bottom: 0,
                        left: 0,
                        transform: 'translateX(-1px)',
                        width: '1px',
                        height: '20px',
                        backgroundColor: '#ccc'
                      }}
                    />
                  </Box>
                  {hours.map((hour, index) => (
                    <Box
                      key={index}
                      id={hour}
                      sx={{
                        height: '60px',
                        border: '1px solid #ccc',
                        position: 'relative',
                        borderTop: 0,
                        borderLeft: 0,
                        zIndex: 3
                      }}
                      draggable
                      onDragStart={handleDragStart(header.name, hour)}
                      onDragEnd={handleDragEnd(header.name)}
                    >
                      {i === 0 && (
                        <Text
                          sx={{
                            position: 'absolute',
                            bottom: 0,
                            left: 0,
                            transform: 'translateX(-75%) translateY(1px)',
                            width: '20px',
                            height: '1px',
                            backgroundColor: '#ccc'
                          }}
                        />
                      )}
                    </Box>
                  ))}
                  {header.items
                    .filter((item) => filterItems(item))
                    .map((item, index) => (
                      <Box
                        key={index}
                        sx={{
                          width: '80%',
                          height: placeItem(item).height,
                          backgroundColor: item.color,
                          position: 'absolute',
                          top: placeItem(item).top,
                          left: '50%',
                          transform: 'translateX(-50%)',
                          borderRadius: '4px',
                          border: '1px solid #ccc',
                          boxShadow: '0 0 4px 0 rgba(0, 0, 0, 0.2)',
                          //   userSelect: 'none',
                          cursor: 'pointer'
                        }}
                      >
                        {item.title}
                      </Box>
                    ))}
                </Box>
              ))}
            </Box>
          </Box>
        </Box>
        <Box
          sx={{
            flex: 4
          }}
        >
          <Box
            sx={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              backgroundColor: 'rgb(203 213 225)',
              paddingX: 5,
              paddingY: 2,
              borderRadius: '10px 10px 0 0'
            }}
          >
            <Text
              sx={{
                fontWeight: 'bold',
                fontSize: '32px'
              }}
            >
              Task list
            </Text>
            <Box
              sx={{
                color: 'green',
                cursor: 'pointer'
              }}
              onClick={() => setIsOpen(true)}
            >
              <DiffAddedIcon size={32} />
            </Box>
          </Box>
          {planData.map((plan, index) => (
            <Box key={index} sx={{ padding: '20px', border: '1px solid #ccc', display: 'flex', gap: 3 }}>
              <Checkbox sx={{ width: '26px', height: '26px' }} />
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Text sx={{ fontSize: '20px', fontWeight: 'bold' }}>{plan.title}</Text>
                <ButtonGroup
                  sx={{
                    display: 'flex',
                    gap: 3
                  }}
                >
                  <Button>
                    <CalendarIcon />
                    <Text sx={{ marginLeft: 2 }}>Today</Text>
                  </Button>
                  <Button
                    style={{
                      color:
                        plan.priority === 'High' ? 'green' : plan.priority === 'Medium' ? 'orange' : 'red'
                    }}
                  >
                    <PinIcon />
                    <Text sx={{ marginLeft: 2 }}>{plan.priority}</Text>
                  </Button>
                  <Button>
                    <TagIcon />
                    <Text sx={{ marginLeft: 2 }}>{plan.type}</Text>
                  </Button>
                </ButtonGroup>
              </Box>
            </Box>
          ))}
          <Dialog isOpen={isOpen} onDismiss={() => setIsOpen(false)}>
            <Dialog.Header
              sx={{
                backgroundColor: '#8ae670'
              }}
            >
              Please enter new planning task!
            </Dialog.Header>
            <Box
              sx={{
                display: 'flex',
                flexDirection: 'column',
                gap: 3,
                padding: 20
              }}
            >
              <TextInput
                placeholder='New task name...'
                value={newTaskName}
                onChange={(e) => setNewTaskName(e.target.value)}
              />
              <Select value={newTaskPriority} onChange={(e) => setNewTaskPriority(e.target.value)}>
                <Select.Option value='High'>High</Select.Option>
                <Select.Option value='Medium'>Medium</Select.Option>
                <Select.Option value='Low'>Low</Select.Option>
              </Select>
              <TextInput
                placeholder='New task type...'
                value={newTaskType}
                onChange={(e) => setNewTaskType(e.target.value)}
              />
              <Box
                sx={{
                  display: 'flex',
                  gap: 2
                }}
              >
                <TextInput
                  value={newStartTime}
                  onChange={(e) => setNewStartTime(e.target.value)}
                  placeholder='Start time...'
                />
                <TextInput
                  value={newEndTime}
                  onChange={(e) => setNewEndTime(e.target.value)}
                  placeholder='End time...'
                />
              </Box>
              <Button
                sx={{
                  '&:hover': {
                    color: 'green',
                    fontWeight: 'bold'
                  }
                }}
                onClick={handleAddTask}
              >
                Add task
              </Button>
            </Box>
          </Dialog>
        </Box>
      </Box>
    </Box>
  );
}
