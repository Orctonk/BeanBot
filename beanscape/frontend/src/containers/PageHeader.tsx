import React from 'react';
import { AppBar, Toolbar, Box } from '@material-ui/core';
import { Link} from 'react-router-dom';
import { makeStyles } from '@material-ui/core/styles';
import { LoginButton } from '../components/LoginButton';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    flexGrow: 1,
    justifyContent: 'space-between'
  },
  title: {
    flexGrow: 1,
    fontSize: '64px',
    fontFamily: 'Audiowide',
    textShadow: '3px 3px #ff00ff',
    color: 'burlywood'
  },
  navigation: {
    position: 'fixed',
    display: 'flex',
    flexGrow: 1,
    flexDirection: 'column',
    top: '81px',
    background: '#3f51b5',
    left: '0px'
  },
  navItem: {
    padding: '10px',
    color: 'white',
    textTransform: 'capitalize',
    fontSize: '24px',
    textDecoration: 'none',
  },
}));

export const PageHeader = () => {
  const classes = useStyles();
  return(
    <div>
      <AppBar className={classes.root}>
        <Toolbar>
          <Link to='/' className={classes.title}>
            <i>Beanscape</i>
          </Link>
          <LoginButton/>
        </Toolbar>
      </AppBar>
      <Box className={classes.navigation}>
        <Link to='/beans' className={classes.navItem}>
          Beans
        </Link>
      </Box>
    </div>
  );
}