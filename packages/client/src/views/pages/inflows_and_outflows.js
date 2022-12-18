import React from 'react';

// material-ui
import { Typography } from '@material-ui/core';

// project imports
import MainCard from 'ui-component/cards/MainCard';

//= =============================|| SAMPLE PAGE ||==============================//

const InflowsAndOutflows = () => (
    <MainCard title="Inflows and Outflows">
        <Typography variant="body2">
            Search for the different inflows and outflows in the market.
        </Typography>
    </MainCard>
);

export default InflowsAndOutflows;
