import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { RootState } from '../store';
import { Message } from '../components/MessageCard';

export interface MessageLogState {
  messages: {
    [id: number]: Message;
  }
  messageId: number;
}

const initialState: MessageLogState = {
  messages: [],
  messageId: 0
};

export const messageLogSlice = createSlice({
  name: 'messageLog',
  initialState,
  reducers: {
    postMessage: (state, action: PayloadAction<Message>) => {
      let msgs = {...state.messages};
      msgs[state.messageId] = action.payload;
      state.messages = msgs;
      state.messageId += 1;
    },
    removeMessage: (state, action: PayloadAction<number>) => {
      console.log(`remove id ${action.payload}`);
      let msgs = {...state.messages};
      if(msgs.hasOwnProperty(action.payload)){
        delete msgs[action.payload];
      }
      state.messages = msgs;
    },
  },
});

export const { postMessage, removeMessage } = messageLogSlice.actions;

export const getMessages = (state: RootState) => state.messageLog.messages;

export default messageLogSlice.reducer;