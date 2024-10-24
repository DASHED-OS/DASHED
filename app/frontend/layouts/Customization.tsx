import React from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { Button, Switch, FormControlLabel } from '@mui/material';

// Assuming there's an action to toggle a dark mode setting
import { toggleDarkMode } from 'store/actions/customizationActions';

const Customization: React.FC = () => {
  const dispatch = useDispatch();
  const customization = useSelector((state: any) => state.customization);

  const handleToggleDarkMode = () => {
    dispatch(toggleDarkMode());
  };

  return (
    <div style={{ padding: '20px' }}>
      <h2>Customization Settings</h2>
      <FormControlLabel
        control={
          <Switch
            checked={customization.darkMode}
            onChange={handleToggleDarkMode}
            name="darkMode"
            color="primary"
          />
        }
        label="Dark Mode"
      />
      <Button variant="contained" color="primary" onClick={handleToggleDarkMode}>
        Toggle Dark Mode
      </Button>
    </div>
  );
};

export default Customization;