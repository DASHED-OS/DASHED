import React from 'react';

// material-ui
import { Typography } from '@material-ui/core';

// project imports
import MainCard from 'ui-component/cards/MainCard';

//= =============================|| SAMPLE PAGE ||==============================//

const OnChainAnalytics = () => (
    <MainCard title="On-Chain Analytics">
        <Typography variant="body2">
        Based on the On-Chain movements we will expect to see changes Off-Chain and vice versa.
        </Typography>
    </MainCard>
);

export default OnChainAnalytics;