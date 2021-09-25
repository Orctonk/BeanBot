import React from 'react';
import { Redirect, useLocation } from 'react-router-dom';
import useStyle from '../styles/GlobalStyle';
import useLocalStorage from '../util/useLocalStorage';

function useQuery() {
  return new URLSearchParams(useLocation().search);
}

export const HomeScreen = () => {
  const classes = useStyle();
  const [count, setCount] = useLocalStorage('beanCount', 0);
  const query = useQuery();

  if(query.get('redirect')){
    return <Redirect to={query.get('redirect') ?? ''}/>
  }

  return (
    <div className="App">
      <header className={classes.content}>
        <button className="BeanButton" onClick={() => setCount(count + 1)}><img src="basic_bean.png" className="App-logo" alt="logo" /></button>
        <i className="TitleText">
          Beans clicked:
        </i>
        <i className="TitleText">
          {count}
        </i>
      </header>
    </div>
  );
};