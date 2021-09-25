import React, { useEffect } from 'react';
import { useAuth0 } from '@auth0/auth0-react';
import { Button, Box, Typography, makeStyles } from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    alignItems: 'center',
  },
  userName: {
    marginRight: theme.spacing(1),
  },
  loginButton: {
    background: 'white',
    textTransform: 'capitalize',
    marginLeft: theme.spacing(1),
  }
}));

export const LoginButton = () => {
  const classes = useStyles();
  const { loginWithRedirect, isAuthenticated, logout, user, getAccessTokenSilently } = useAuth0();

  useEffect(() => {
    getAccessTokenSilently().catch(() => {});
  },[getAccessTokenSilently]);

  if(!isAuthenticated)
    return <Button className={classes.loginButton} onClick={() => loginWithRedirect()}>Log In</Button>;
  return (
    <Box className={classes.root}>
      <Typography variant="h5" className={classes.userName}>{user?.name}</Typography>
      <Button 
        className={classes.loginButton} 
        onClick={() => 
          logout({
            returnTo:`${window.location.origin}?redirect=${window.location.pathname}`
          })
        }>
        Log out
      </Button>
    </Box>
  )
};