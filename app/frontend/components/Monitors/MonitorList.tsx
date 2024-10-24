import React from 'react';
import { useMonitors } from './MonitorContext';
import MonitorItem from './MonitorItem';

const MonitorList: React.FC = () => {
  const { monitors } = useMonitors();

  return (
    <div>
      <h2>Monitor List</h2>
      {monitors.length > 0 ? (
        monitors.map((monitor) => (
          <MonitorItem
            key={monitor.id}
            id={monitor.id}
            name={monitor.name}
            status={monitor.status}
            onSelect={(id) => console.log(`Selected monitor with id: ${id}`)}
          />
        ))
      ) : (
        <p>No monitors available.</p>
      )}
    </div>
  );
};

export default MonitorList;