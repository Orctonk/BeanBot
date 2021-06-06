import { configureStore, ThunkAction, Action } from '@reduxjs/toolkit';
import messageLogReducer from './slices/MessageLogSlice';

export const store = configureStore({
  reducer: {
    messageLog: messageLogReducer,
  },
});

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
export type AppThunk<ReturnType = void> = ThunkAction<
  ReturnType,
  RootState,
  unknown,
  Action<string>
>;