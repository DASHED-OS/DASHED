import React from 'react';

// material-ui
import { Typography } from '@material-ui/core';

// project imports
import MainCard from 'ui-component/cards/MainCard';

//= =============================|| SAMPLE PAGE ||==============================//

const RunningStats = () => (
    <MainCard title="Running Stats">
        <Typography variant="body2">
            This will show the live changes happening on the different 
            Flashloan Bots based on AI Market Makers. These will be 
            operating in three sections Futures, Opitons and Flashloans. 
            All on borrowed money, all borrowed for no leverage cost and 
            a total sum cost of zero.
        </Typography>
    </MainCard>
);

export default RunningStats;
