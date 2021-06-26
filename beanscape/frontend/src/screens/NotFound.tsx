import React from 'react';
import useStyle from '../styles/GlobalStyle';

export const NotFound = () => {
  const classes = useStyle();

  return (
    <div className="App">
      <header className={classes.content}>
        <div><img src="basic_bean.png" className="App-logo" alt="logo" /></div>
        <i className="TitleText">
          Page not found
        </i>
      </header>
    </div>
  );
};