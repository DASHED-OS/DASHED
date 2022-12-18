import React from 'react';

// material-ui
import { Typography } from '@material-ui/core';

// project imports
import MainCard from 'ui-component/cards/MainCard';

//= =============================|| SAMPLE PAGE ||==============================//

const OffChainAnalytics = () => (
    <MainCard title="Off-Chain Analytics">
        <Typography variant="body2">
            Based on the Off-Chain movements we will expect to see changes On-Chain and vice versa.
        </Typography>
    </MainCard>
);

export default OffChainAnalytics;