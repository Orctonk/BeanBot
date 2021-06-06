import React, {useEffect, useState} from 'react';
import {Card, CardMedia, Grid, ButtonBase, Typography} from '@material-ui/core';
import DeleteIcon from '@material-ui/icons/Delete';
import AddIcon from '@material-ui/icons/Add';
import LinkIcon from '@material-ui/icons/Link';
import FileCopyOutlined from '@material-ui/icons/FileCopy';
import { useAuth0 } from '@auth0/auth0-react';
import useStyle from '../styles/GlobalStyle';
import axios from 'axios';
import { AddBeanModal, prettyBeanName, beanFileName } from '../components/AddBeanModal';
import { postMessage, useAppDispatch } from '../util/PostMessage';

export const BeanScreen = () => {
  const classes = useStyle()
  const dispatch = useAppDispatch();
  const { isAuthenticated, getAccessTokenSilently } = useAuth0();
  const [ beans, setBeans ] = useState<string[]>();
  const [ token, setToken ] = useState<string>();
  const [ addBeanModalOpen, setAddBeanOpen ] = useState<boolean>(false);

  useEffect(() => {
    axios
      .get(`${process.env.REACT_APP_API}/beans`)
      .then((response) => setBeans(response.data))
      .catch((err) => setBeans([]));
  }, []);

  useEffect(() => {
    getAccessTokenSilently()
      .then((token) => {
        setToken(token);
      })
      .catch((err) => {
        console.log(err);
      });
  },[getAccessTokenSilently]);

  const handleBeanDelete = (bean: string) => {
    axios
      .delete(`${process.env.REACT_APP_API}/beans/${bean}`, {
        headers: {
          Authorization: `Bearer ${token}`
        }
      })
      .then(() => {
        window.location.reload();
      })
      .catch((err) => {
        postMessage(dispatch, 'Failed to delete bean!', 'error');
      });
  }

  const handleAddBean = (name:string, file: File) => {
    file.arrayBuffer().then((buffer) => {
      axios
      .post(`${process.env.REACT_APP_API}/beans?name=${beanFileName(name)}`, 
      buffer,
      {
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'image/png'
        }
      })
      .then(() => {
        window.location.reload();
      })
      .catch((err) => {
        if(err.response.status === 409){
          postMessage(dispatch, 'A bean with that name already exists', 'error');
        } else {
          postMessage(dispatch, 'Failed to add bean!', 'error');
          console.log(err.response);
        }
      });
    })
  };

  const getCards = () => {
    const cards: JSX.Element[] = [];
    if(!beans) return [];
    beans.forEach((bean: string) => {
      cards.push(
        <Card className={classes.beanCard} key={bean}>
          <CardMedia className={classes.beanCardButtonBar}>
            <ButtonBase href={`${process.env.REACT_APP_API}/beans/${bean}`}><LinkIcon/></ButtonBase>
            <ButtonBase onClick={() => {
              navigator.clipboard.writeText(`${process.env.REACT_APP_API}/beans/${bean}`)
              postMessage(dispatch, `${prettyBeanName(bean)} copied to clipboard`);
            }}><FileCopyOutlined/></ButtonBase>
            {isAuthenticated && 
              <ButtonBase onClick={() => handleBeanDelete(bean)}>
                <CardMedia component={DeleteIcon}></CardMedia>
              </ButtonBase>
            }
          </CardMedia>
          <CardMedia component="img" src={`${process.env.REACT_APP_API}/beans/${bean}`}/>
          <CardMedia><Typography variant="h5">{prettyBeanName(bean)}</Typography></CardMedia>
        </Card>
      );
    });
    return cards;
  };

  return (
    <div className={classes.content}>
      <AddBeanModal 
        open={addBeanModalOpen} 
        onClose={() => setAddBeanOpen(false)}
        onSave={handleAddBean}  
      />
      <Grid container className={classes.beanGrid}>
        { isAuthenticated && 
          <Card className={classes.addBeanCard}>
            <ButtonBase onClick={() => setAddBeanOpen(true) }>
              <CardMedia component={AddIcon} style={{fontSize: '200px'}}></CardMedia>
            </ButtonBase>
          </Card>
        }
        { getCards() }
      </Grid>
    </div>
  );
};