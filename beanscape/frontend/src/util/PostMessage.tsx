import { useDispatch } from 'react-redux';
import { postMessage as postAction } from '../slices/MessageLogSlice';
import { AppDispatch } from '../store';

export const postMessage = (dispatch: AppDispatch,text: string, type: 'error' | 'warning' | 'message' = 'message') => {
  dispatch(postAction({
    type,
    text
  }));
}

export const useAppDispatch = () => useDispatch<AppDispatch>();