import React, { useEffect } from 'react';
import { ButtonBase, Card, CardMedia, makeStyles } from '@material-ui/core';
import CloseIcon from '@material-ui/icons/Close';

const useStyles = makeStyles((theme) => ({
  messageCard:{
    display: 'flex',
    justifyContent: 'space-between',
    width: '30vw',
    padding: '10px',
    margin: '10px',
  },
  error: {
    background: '#ff4040'
  },
  warning: {
    background: '#ffff66'
  },
  message: {
    background: '#99ccff'
  }
}));

export interface Message {
  type: "error" | "warning" | "message",
  text: string
}

export interface Props {
  message: Message,
  onRemove: () => void,
};

export const MessageCard = ({message, onRemove} : Props) => {
  const classes = useStyles();
  useEffect(() => {
    setTimeout(()=>{
      onRemove();
    },5000)
  });
  
  return (
    <Card className={`${classes.messageCard} ${classes[message.type]}`}>
      <CardMedia>{message.text}</CardMedia>
      <CardMedia><ButtonBase onClick={onRemove}><CloseIcon/></ButtonBase></CardMedia>
    </Card>
  );
}