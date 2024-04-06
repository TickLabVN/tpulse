import { useState, useEffect } from 'react';
import { TextInput, Text, Box, Button, ButtonGroup, Radio } from '@primer/react';
import { Resizable } from 're-resizable';
import { TaskDialog } from './TaskDialog';
// import { homeDir } from '@tauri-apps/api/path';
// import { getRandomColor } from '@utils';
import Database from 'tauri-plugin-sql-api';

import {
  CalendarIcon,
  PinIcon,
  TagIcon,
  ArrowLeftIcon,
  ArrowRightIcon,
  PlusIcon
} from '@primer/octicons-react';
import moment from 'moment';
import { formatTime, convertTimeToNumber, convertNumberToTime } from '@utils';
//import { useZoom } from '@hooks';

interface activity {
  title: string;
  start: string;
  end: string;
  category_tag: string;
  color: string;
}

interface Item {
  title: string;
  start: string;
  end: string;
  color: string;
}
export function DayView() {
  const [items, setItems] = useState<activity[]>([]);
  useEffect(() => {
    const initDatabase = async () => {
      try {
        const dbPath = '/home/tan17112003/.ticklabvn.tpulse/tpulse.sqlite3';
        const db = await Database.load(`sqlite:${dbPath}`);
        const result = await db.select('SELECT title, start, end, category_tag FROM activity_log');
        setItems(result as activity[]);
      } catch (error) {
        alert(error);
      }
    };
    initDatabase();
  }, []);

  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [elapsedTime, setElapsedTime] = useState<number>(0);

  useEffect(() => {
    let interval: NodeJS.Timeout;

    if (isRunning) {
      interval = setInterval(() => {
        setElapsedTime((prevTime) => prevTime + 1);
      }, 1000);
    }

    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  }, [isRunning]);

  const toggleTimer = () => {
    setIsRunning(!isRunning);
  };
  const [mousePosition, setMousePosition] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const [divHeight, setDivHeight] = useState<number>(0);
  const [divWidth, setDivWidth] = useState<number>(0);
  const [isResizing, setIsResizing] = useState<boolean>(false);
  const [divLeft, setDivLeft] = useState<number>(0);
  const [divTop, setDivTop] = useState<number>(0);

  const [newTaskName, setNewTaskName] = useState<string>('');
  const [newTaskPriority, setNewTaskPriority] = useState<string>('');
  const [newTaskType, setNewTaskType] = useState<string>('');
  const [newStartTime, setNewStartTime] = useState<string>('');
  const [newEndTime, setNewEndTime] = useState<string>('');
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [startElement, setStartElement] = useState<number>(0);
  const [startWidenElement, setStartWidenElement] = useState<number>(0);

  const handleWidenStart = (e: React.MouseEvent<HTMLElement, MouseEvent> | React.TouchEvent<HTMLElement>) => {
    let topEdgeElement = 0;
    if ('clientY' in e) {
      topEdgeElement = e.clientY;
    } else {
      topEdgeElement = e.touches[0].clientY;
    }
    setStartWidenElement(topEdgeElement);
    setIsResizing(false);
  };
  const handleWidenEnd = (end: string, index: number) => (e: MouseEvent | TouchEvent) => {
    let bottomEdgeElement = 0;
    if (e instanceof MouseEvent) {
      bottomEdgeElement = e.clientY;
    }
    if (e instanceof TouchEvent) {
      bottomEdgeElement = e.touches[0].clientY;
    }
    const distance = bottomEdgeElement - startWidenElement;
    const eachTimeRange = Math.round(convertTimeToNumber(hours[1]) - convertTimeToNumber(hours[0])) / 60; // 1px ung voi bao nhieu gio (gio/px)
    const timeRange = distance * eachTimeRange;
    const newEndTime = convertNumberToTime(convertTimeToNumber(end) + timeRange);
    const newPlanData = [...planData];
    newPlanData[index].end = newEndTime;
    setPlanData(newPlanData);
    setIsResizing(false);
  };

  const handleDragStart = (e: React.DragEvent<HTMLDivElement>) => {
    e.currentTarget.style.opacity = '0.5';
    setStartElement(e.clientY);
  };
  const handleDragEnd =
    (start: string, end: string, index: number) => (e: React.DragEvent<HTMLDivElement>) => {
      e.currentTarget.style.opacity = '1';
      const endElement = e.clientY;
      const distance = endElement - startElement; // khoang cach giua 2 diem start va end khi drag (px)
      const eachTimeRange = Math.round(convertTimeToNumber(hours[1]) - convertTimeToNumber(hours[0])) / 60; // 1px ung voi bao nhieu gio (gio/px)
      const timeRange = distance * eachTimeRange;
      const newPlanningStartTime = convertNumberToTime(convertTimeToNumber(start) + timeRange);
      const newPlanningEndTime = convertNumberToTime(convertTimeToNumber(end) + timeRange);
      const newPlanData = [...planData];
      newPlanData[index].start = newPlanningStartTime;
      newPlanData[index].end = newPlanningEndTime;
      setPlanData(newPlanData);
      setIsResizing(false);
    };

  const handleMouseClick = () => {
    // setIsResizing((prev) => !prev);
  };
  const handleMouseMove = (e: MouseEvent) => {
    if (isResizing) {
      const newWidth = e.clientX - divLeft;
      const newHeight = e.clientY - divTop;
      setDivWidth(newWidth > 0 ? newWidth : 0);
      setDivHeight(newHeight > 0 ? newHeight : 0);
    }
  };
  const handleMouseUp = (handleName: string, hour: string) => (e: MouseEvent) => {
    setIsResizing(true);
    setDivLeft(e.clientX);
    setDivTop(e.clientY);
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
      if (handleName === 'Activity') {
        setStartTime(exactHourToString);
      } else {
        setNewStartTime(exactHourToString);
      }
    } else {
      alert('Error');
    }
  };
  const handleMouseDown = (handleName: string) => (e: MouseEvent) => {
    setIsResizing(false);
    setDivWidth((prev) => prev * 0);
    setDivHeight((prev) => prev * 0);
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
      if (handleName === 'Activity') {
        handleSubmitRange(startTime, exactHourToString);
        setStartTime('');
      } else {
        setNewEndTime(exactHourToString);
        if (newStartTime && newEndTime) {
          setMousePosition({ x: e.pageX, y: e.pageY });
          setIsOpen(true);
          setIsResizing(false);
        }
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
    setIsResizing(false);
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
  const hour = Array.from({ length: 24 }, (_, i) =>
    moment().utcOffset('GMT+7').startOf('day').add(i, 'hours').format('HH:mm')
  );
  const [hours, setHours] = useState<string[]>(hour);
  const colors = {
    gray: 'rgb(97, 97, 97)',
    orange: 'rgb(244, 81, 30)',
    blue: 'rgb(63, 81, 181)',
    violet: 'rgb(142, 36, 170)',
    red: 'rgb(213, 0, 0)'
  };

  const [planData, setPlanData] = useState([
    {
      id: 1,
      title: 'Conceptual design',
      priority: 'High',
      type: 'Work',
      start: '7:00:00.000',
      end: '8:00:00.000',
      color: colors.red
    },
    {
      id: 2,
      title: 'Requirement Specification',
      priority: 'Medium',
      type: 'Work',
      start: '12:00:00.000',
      end: '18:00:00.000',
      color: colors.blue
    },
    {
      id: 3,
      title: 'Media planning',
      priority: 'Medium',
      type: 'Work',
      start: '14:00:00.000',
      end: '16:00:00.000',
      color: colors.violet
    }
  ]);
  const headers = [
    {
      name: 'Activity',
      items: items
    },
    {
      name: 'Planning',
      items: planData
    }
  ];

  const [startTime, setStartTime] = useState<string>('');
  const [endTime, setEndTime] = useState<string>('');

  const handleSubmitRange = (startInput: string, endInput: string) => {
    /* Extract hours, minutes, and seconds from the input values */
    const parsedStartTime = convertTimeToNumber(startInput);
    const parsedEndTime = convertTimeToNumber(endInput);
    if (isNaN(parsedStartTime) || isNaN(parsedEndTime) || parsedStartTime >= parsedEndTime) {
      // Handle invalid input or show an error message
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
    <>
      {/* <Text>{homeDirectory}</Text> */}
      <Box
        sx={{
          position: 'absolute',
          top: `${divTop + window.scrollY}px`,
          left: `${divLeft}px`,
          width: `${divWidth}px`,
          height: `${divHeight}px`,
          backgroundColor: 'black',
          zIndex: 100,
          opacity: 0.5,
          pointerEvents: 'none'
        }}
      ></Box>
      <Box
        sx={{
          padding: '50px',
          backgroundColor: '#E5E5E5',
          display: 'flex',
          gap: 4
        }}
      >
        <Box
          sx={{
            flex: 9,
            backgroundColor: 'white',
            padding: '40px'
          }}
        >
          <Text
            sx={{
              fontSize: '28px',
              fontWeight: '500',
              lineHeight: '28px',
              borderBottom: '1px solid #ccc',
              display: 'block',
              paddingBottom: 3
            }}
          >
            Homepage
          </Text>
          <Box
            sx={{
              marginTop: '32px',
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              marginBottom: '20px'
            }}
          >
            <Text
              sx={{
                fontWeight: 'bold'
              }}
            >
              Time Tracking
            </Text>
            <Box
              sx={{
                display: 'flex',
                alignItems: 'center',
                gap: 4
              }}
            >
              <Text
                sx={{
                  opacity: '70%'
                }}
              >
                {`${moment().format('dddd')}, ${moment().format('L')}`}
              </Text>
              <Button>Today</Button>
              <ButtonGroup>
                <Button>
                  <ArrowLeftIcon />
                </Button>
                <Button>
                  <CalendarIcon />
                </Button>
                <Button>
                  <ArrowRightIcon />
                </Button>
              </ButtonGroup>
            </Box>
          </Box>
          <Box
            sx={{
              marginTop: 2,
              display: 'flex',
              columnGap: 4
            }}
          >
            <TextInput
              type='time'
              sx={{
                width: '200px',
                height: '40px',
                border: '1px solid #ccc'
              }}
              onChange={(e) => setStartTime(e.target.value)}
            />
            <TextInput
              type='time'
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
                backgroundColor: 'rgb(163 230 53)'
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
                backgroundColor: 'rgb(163 230 53)'
              }}
              onClick={() => setHours(hour)}
            >
              Reset
            </Button>
          </Box>
          <Box
            // ref={zoomRef}
            sx={{
              height: 'fit-content',
              overflowY: 'auto',
              padding: '50px'
            }}
          >
            <Box
              sx={{
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
                    gridTemplateColumns: '1fr 1fr',
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
                        textAlign: 'left',
                        position: 'relative'
                      }}
                    >
                      <Box
                        sx={{
                          padding: 2,
                          borderBottom: '1px solid #ccc',
                          position: 'relative',
                          fontWeight: 'bold',
                          backgroundColor: 'rgb(228 228 231)',
                          borderRadius: '5px 5px 0 0'
                        }}
                      >
                        {header.name}
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
                            borderLeft: 0
                            // zIndex: 3
                          }}
                          onClick={handleMouseClick}
                          onMouseUp={!isResizing && handleMouseUp(header.name, hour)}
                          onMouseMove={isResizing && handleMouseMove}
                          onMouseDown={handleMouseDown(header.name)}
                        ></Box>
                      ))}
                      {header.items
                        .filter((item) => filterItems(item))
                        .map((item, index) => (
                          <Resizable
                            size={{
                              width: '80%',
                              height: placeItem(item).height
                            }}
                            style={{
                              position: 'absolute',
                              top: placeItem(item).top,
                              left: '50%',
                              transform: 'translateX(-50%)',
                              borderRadius: '8px',
                              border: '1px solid #ccc',
                              color: 'white',
                              cursor: 'pointer',
                              borderLeft: '5px solid rgb(66, 133, 244)',
                              pointerEvents: header.name === 'Planning' ? 'auto' : 'none'
                            }}
                            key={index}
                            enable={{
                              top: false,
                              right: false,
                              bottom: header.name === 'Planning' ? true : false,
                              left: false,
                              topRight: false,
                              bottomRight: false,
                              bottomLeft: false,
                              topLeft: false
                            }}
                            onResizeStart={handleWidenStart}
                            onResizeStop={handleWidenEnd(item.end, index)}
                          >
                            <Box
                              height='100%'
                              sx={{
                                padding: '5px',
                                backgroundColor: header.name === 'Planning' ? item.color : 'red',
                                pointerEvents: header.name === 'Planning' ? 'auto' : 'none'
                              }}
                              draggable={header.name === 'Planning' ? true : false}
                              onDragStart={header.name === 'Planning' && handleDragStart}
                              onDragEnd={
                                header.name === 'Planning' && handleDragEnd(item.start, item.end, index)
                              }
                            >
                              <Box
                                sx={{
                                  display: 'flex',
                                  flexDirection: 'column',
                                  fontWeight: 'bold'
                                }}
                              >
                                <Text>{item.title}</Text>
                                {header.name === 'Planning' && 'type' in item && (
                                  <Text>{`Type: ${item.type}`}</Text>
                                )}
                                <Text>
                                  {item.start.split(':')[0] + ':' + item.start.split(':')[1]} -{' '}
                                  {item.end.split(':')[0] + ':' + item.end.split(':')[1]}
                                </Text>
                              </Box>
                            </Box>
                          </Resizable>
                        ))}
                    </Box>
                  ))}
                </Box>
              </Box>
            </Box>
          </Box>
        </Box>
        <Box
          sx={{
            flex: 3
          }}
        >
          <Box
            sx={{
              padding: 2,
              borderBottom: '1px solid #ccc',
              backgroundColor: 'white'
            }}
          >
            <Text
              sx={{
                fontWeight: 'bold',
                fontSize: '28px',
                display: 'block',
                borderBottom: '1px solid #ccc',
                marginBottom: '32px'
              }}
            >
              Session timer
            </Text>
            <Box
              sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center'
              }}
            >
              <Text>Time Elapse</Text>
              <Text
                sx={{
                  fontSize: '40px',
                  fontWeight: 'bold'
                }}
              >
                {formatTime(elapsedTime)}
              </Text>
            </Box>
          </Box>
          <Button
            sx={{
              backgroundColor: 'black',
              paddingY: 5,
              borderRadius: 0,
              color: 'white',
              width: '100%',
              fontWeight: 'bold',
              marginBottom: 5
            }}
            onClick={toggleTimer}
          >
            {isRunning ? 'Stop Session' : 'Start Session'}
          </Button>
          <Box
            sx={{
              paddingX: 5,
              paddingY: 2,
              backgroundColor: 'white'
            }}
          >
            <Text
              sx={{
                fontWeight: 'bold',
                fontSize: '32px'
              }}
            >
              Today Task
            </Text>
            {planData.map((plan, index) => (
              <Box
                key={index}
                sx={{
                  padding: '20px',
                  border: '1px solid #ccc',
                  display: 'flex',
                  gap: 3,
                  backgroundColor: 'rgb(228 228 231)',
                  marginBottom: 5
                }}
              >
                <Radio sx={{ width: '26px', height: '26px' }} value='default' name='item' />
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
            <Box
              sx={{
                display: 'flex',
                alignItems: 'center',
                opacity: '70%',
                cursor: 'pointer'
              }}
              onClick={() => setIsOpen(true)}
            >
              <PlusIcon size={32} />
              <Text
                sx={{
                  fontSize: 4
                }}
              >
                Add a task
              </Text>
            </Box>
          </Box>
          {isOpen && (
            <TaskDialog
              handleAddTask={handleAddTask}
              setNewTaskName={setNewTaskName}
              setNewTaskType={setNewTaskType}
              setNewTaskPriority={setNewTaskPriority}
              setNewStartTime={setNewStartTime}
              setNewEndTime={setNewEndTime}
              setIsOpen={setIsOpen}
              mousePosition={mousePosition}
              setIsResizing={setIsResizing}
            />
          )}
        </Box>
      </Box>
    </>
  );
}
