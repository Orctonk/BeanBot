import React from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { Box, makeStyles } from '@material-ui/core';
import { Message, MessageCard } from '../components/MessageCard';
import { getMessages, removeMessage } from '../slices/MessageLogSlice';
import { AppDispatch } from '../store';

const useStyles = makeStyles((theme) => ({
  messageLog:{
    position: 'fixed',
    bottom: 0,
    right: 0,
  },
  messageCard:{
    width: '30vw',
    padding: '10px',
    margin: '10px',
    background: '#ff4040'
  }
}));

export const MessageLog = () => {
  const classes = useStyles();
  const messages = useSelector(getMessages);
  const dispatch = useDispatch<AppDispatch>();

  return(
    <Box className={classes.messageLog}>
      {
        Object.entries(messages).map((id: [string, Message]) => 
          <MessageCard key={id[0]} message={id[1]} onRemove={()=>{dispatch(removeMessage(parseInt(id[0])))}}/>
        )
      }
    </Box>
  );
}