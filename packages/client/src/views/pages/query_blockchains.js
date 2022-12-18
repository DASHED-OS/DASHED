import React from 'react';

// material-ui
import { Typography } from '@material-ui/core';

// project imports
import MainCard from 'ui-component/cards/MainCard';

//= =============================|| QUERY PAGE ||==============================//

const QueryBlockchain = () => (
    <MainCard title="Pick A Blockchain">
        <Typography variant="body2">
            Choose which blockchain you're going to query, which will also
            be reflected in the list of queries available. This will have 
            to be set-up using macros. 
        </Typography>
    </MainCard>
);

export default QueryBlockchain;