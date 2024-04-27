// /* eslint-disable no-unused-vars */
// import { XIcon, ClockIcon, LocationIcon, ListOrderedIcon } from '@primer/octicons-react';
// import { TimePicker } from 'antd';
// import { Box, Button, IconButton, TextInput, Text, RadioGroup, FormControl, Radio } from '@primer/react';

// interface TaskDialogProps {
//   handleAddTask: () => void;
//   setNewTaskName: (taskName: string) => void;
//   setNewTaskType: (taskType: string) => void;
//   setNewTaskPriority: (taskPriority: string) => void;
//   setNewStartTime: (startTime: string) => void;
//   setNewEndTime: (endTime: string) => void;
//   setIsOpen: (isOpen: boolean) => void;
//   mousePosition: { x: number; y: number };
//   setIsResizing: (isResizing: boolean) => void;
// }
// export const TaskDialog = ({
//   handleAddTask,
//   setNewTaskName,
//   setNewTaskType,
//   setNewTaskPriority,
//   setNewStartTime,
//   setNewEndTime,
//   setIsOpen,
//   mousePosition,
//   setIsResizing
// }: TaskDialogProps) => {
//   const TaskType = ['Event', 'Focus Time', 'Absence', 'Meeting', 'Task'];
//   const TaskPriority = ['Low', 'Medium', 'High'];
//   return (
//     <Box
//       sx={{
//         backgroundColor: 'white',
//         width: '400px',
//         margin: '0 auto',
//         borderRadius: '10px',
//         boxShadow: '0 0 20px rgba(0, 0, 0, 0.2)',
//         paddingBottom: 5,
//         position: 'absolute',
//         left: mousePosition.x,
//         top: mousePosition.y
//       }}
//     >
//       <Box
//         sx={{
//           height: '36px',
//           display: 'flex',
//           flexDirection: 'row-reverse',
//           alignItems: 'center',
//           width: '100%',
//           backgroundColor: 'rgb(218,220,224)'
//         }}
//       >
//         <IconButton
//           icon={XIcon}
//           aria-label='Close'
//           sx={{
//             backgroundColor: 'transparent',
//             border: 'none',
//             marginRight: '16px',
//             boxShadow: 'none',
//             borderRadius: '50%'
//           }}
//           onClick={() => {
//             setIsOpen(false);
//             setIsResizing(false);
//           }}
//         />
//       </Box>
//       <Box sx={{ padding: '20px' }}>
//         <TextInput
//           sx={{
//             width: '100%',
//             height: '40px',
//             border: 0,
//             borderBottom: '1px solid #ccc',
//             marginBottom: '10px',
//             padding: 0,
//             boxShadow: 'none',
//             borderRadius: 0,
//             ':focus-within': {
//               borderBottom: '1px solid blue'
//             },
//             fontSize: '20px',
//             fontWeight: 'bold'
//           }}
//           onChange={(e) => setNewTaskName(e.target.value)}
//           placeholder='Add your title...'
//         />
//       </Box>
//       <Box>
//         <Box
//           sx={{
//             display: 'flex',
//             gap: 1,
//             margin: '0 16px'
//           }}
//         >
//           {TaskType.map((type) => (
//             <Button
//               key={type}
//               sx={{
//                 backgroundColor: 'inherit',
//                 border: 'none'
//               }}
//               onClick={() => setNewTaskType(type)}
//             >
//               {type}
//             </Button>
//           ))}
//         </Box>
//         <Box
//           sx={{
//             margin: '16px 16px 20px 16px',
//             display: 'flex',
//             flexDirection: 'column',
//             gap: '16px'
//           }}
//         >
//           <Box sx={{ display: 'flex', alignItems: 'center' }}>
//             <Box
//               sx={{
//                 display: 'flex',
//                 justifyContent: 'center',
//                 width: '36px',
//                 paddingRight: '8px',
//                 flexShrink: 0
//               }}
//             >
//               <ClockIcon size={24} />
//             </Box>
//             <Box
//               sx={{
//                 display: 'flex',
//                 alignItems: 'center',
//                 gap: '16px',
//                 flexGrow: 1,
//                 padding: '0 8px'
//               }}
//             >
//               <Box
//                 sx={{
//                   display: 'flex',
//                   alignItems: 'center',
//                   gap: '8px'
//                 }}
//               >
//                 <Text style={{ fontSize: '16px', fontWeight: 'bold' }}>Start:</Text>
//                 <TimePicker
//                   format='HH:mm:ss'
//                   onChange={(_, timeString) => {
//                     if (typeof timeString === 'string') setNewStartTime(timeString);
//                   }}
//                 />
//               </Box>
//               <Box
//                 sx={{
//                   alignItems: 'center',
//                   gap: '8px'
//                 }}
//               >
//                 <Text style={{ fontSize: '16px', fontWeight: 'bold' }}>End:</Text>
//                 <TimePicker
//                   format='HH:mm:ss'
//                   onChange={(_, timeString) => {
//                     if (typeof timeString === 'string') setNewEndTime(timeString);
//                   }}
//                 />
//               </Box>
//             </Box>
//           </Box>
//           <Box sx={{ display: 'flex', alignItems: 'center' }}>
//             <Box
//               sx={{
//                 display: 'flex',
//                 justifyContent: 'center',
//                 width: '36px',
//                 paddingRight: '8px',
//                 flexShrink: 0
//               }}
//             >
//               <LocationIcon size={24} />
//             </Box>
//             <Box
//               sx={{
//                 display: 'flex',
//                 alignItems: 'center',
//                 gap: '16px',
//                 flexGrow: 1,
//                 padding: '0 8px'
//               }}
//             >
//               <TextInput
//                 sx={{
//                   height: '40px',
//                   border: 0,
//                   borderBottom: '1px solid #ccc',
//                   padding: 0,
//                   boxShadow: 'none',
//                   borderRadius: 0,
//                   ':focus-within': {
//                     borderBottom: '1px solid blue'
//                   },
//                   fontSize: '16px',
//                   fontWeight: 'bold'
//                 }}
//                 block={true}
//                 placeholder='Add your location'
//               />
//             </Box>
//           </Box>
//           <Box sx={{ display: 'flex', alignItems: 'center' }}>
//             <Box
//               sx={{
//                 display: 'flex',
//                 justifyContent: 'center',
//                 width: '36px',
//                 paddingRight: '8px',
//                 flexShrink: 0
//               }}
//             >
//               <ListOrderedIcon size={24} />
//             </Box>
//             <Box
//               sx={{
//                 display: 'flex',
//                 alignItems: 'center',
//                 gap: '16px',
//                 flexGrow: 1,
//                 padding: '0 8px'
//               }}
//             >
//               <RadioGroup
//                 name='priorityRadioGroup'
//                 sx={{
//                   display: 'flex',
//                   flexDirection: 'row',
//                   gap: '16px',
//                   width: '100%'
//                 }}
//               >
//                 <Box
//                   sx={{
//                     display: 'flex',
//                     alignItems: 'center',
//                     gap: '16px'
//                   }}
//                 >
//                   <RadioGroup.Label
//                     sx={{
//                       fontSize: '16px',
//                       fontWeight: 'bold'
//                     }}
//                   >
//                     Priority:
//                   </RadioGroup.Label>
//                   {TaskPriority.map((priority) => (
//                     <FormControl key={priority} sx={{ display: 'flex', alignItems: 'center', width: '100%' }}>
//                       <Radio value={priority} onClick={() => setNewTaskPriority(priority)} />
//                       <FormControl.Label>{priority}</FormControl.Label>
//                     </FormControl>
//                   ))}
//                 </Box>
//               </RadioGroup>
//             </Box>
//           </Box>
//         </Box>
//       </Box>
//       <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: '16px', padding: '0 16px' }}>
//         <Button
//           sx={{
//             backgroundColor: 'transparent',
//             border: '1px solid #ccc',
//             color: '#ccc'
//           }}
//           onClick={() => {
//             setIsOpen(false);
//             setIsResizing(false);
//           }}
//         >
//           Cancel
//         </Button>
//         <Button
//           sx={{
//             backgroundColor: '#2ea44f',
//             border: 'none',
//             color: 'white'
//           }}
//           onClick={handleAddTask}
//         >
//           Save
//         </Button>
//       </Box>
//     </Box>
//   );
// };
