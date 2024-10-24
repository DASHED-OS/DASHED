import React, { useState, ReactNode } from 'react';
import { MonitorContext } from './MonitorContext';

interface MonitorProviderProps {
  children: ReactNode;
}

interface Monitor {
  id: string;
  name: string;
  status: string;
}

const MonitorProvider: React.FC<MonitorProviderProps> = ({ children }) => {
  const [monitors, setMonitors] = useState<Monitor[]>([]);

  const addMonitor = (monitor: Monitor) => {
    setMonitors((prevMonitors) => [...prevMonitors, monitor]);
  };

  const removeMonitor = (id: string) => {
    setMonitors((prevMonitors) => prevMonitors.filter(monitor => monitor.id !== id));
  };

  return (
    <MonitorContext.Provider value={{ monitors, addMonitor, removeMonitor }}>
      {children}
    </MonitorContext.Provider>
  );
};

export default MonitorProvider;