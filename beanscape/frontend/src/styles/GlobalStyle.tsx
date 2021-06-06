import { makeStyles } from '@material-ui/core/styles';

export default makeStyles((theme) => ({
  content:{
    display: 'flex',
    flexDirection: 'column',
    backgroundColor: '#282c34',
    padding: '80px 90px 0px 90px',
    minHeight: '100vh',
    color: 'white',
  },
  beanGrid: {
    justifyContent: 'center',
  },
  beanCard: {
    backgroundColor: 'white',
    width: '200px',
    height: 'auto',
    margin: '10px',
    fontSize: '24px',
    textAlign: 'center'
  },
  beanCardButtonBar: {
    display: 'flex',
    justifyContent: 'space-between',
    margin: '10px',
  },
  addBeanCard: {
    display: 'flex',
    backgroundColor: 'white',
    width: '200px',
    margin: '10px',
    alignContent: 'center'
  },
  modalBox: {
    display: 'flex',
    flexDirection: 'column',
    padding: '50px 50px 20px 50px',
    position: 'absolute',
    background: 'white',
    textAlign: 'center',
    top: '50%', 
    left: '50%', 
    transform: 'translate(-50%, -50%)'
  },
  modalSave: {
    margin: '20px 0px 0px 0px',
    background: 'linear-gradient(45deg, #00bBbB 30%, #008E53 90%)'
  },
  modalCancel: {
    margin: '20px 0px 0px 10px',
    background: 'linear-gradient(45deg, #ffbBbB 30%, #ff8E53 90%)'
  }
}));