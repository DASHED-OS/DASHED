import React from 'react';

interface MonitorActionsProps {
  onAddMonitor: () => void;
  onRemoveMonitor: (id: string) => void;
  onUpdateMonitor: (id: string) => void;
}

const MonitorActions: React.FC<MonitorActionsProps> = ({ onAddMonitor, onRemoveMonitor, onUpdateMonitor }) => {
  return (
    <div>
      <h2>Monitor Actions</h2>
      <button onClick={onAddMonitor}>Add Monitor</button>
      <button onClick={() => onRemoveMonitor('example-id')}>Remove Monitor</button>
      <button onClick={() => onUpdateMonitor('example-id')}>Update Monitor</button>
    </div>
  );
};

export default MonitorActions;