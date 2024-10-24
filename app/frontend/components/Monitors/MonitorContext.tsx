import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the monitor context
interface MonitorContextType {
  monitors: Array<{ id: string; name: string; status: string }>;
  addMonitor: (monitor: { id: string; name: string; status: string }) => void;
  removeMonitor: (id: string) => void;
}

// Create the context with a default value
const MonitorContext = createContext<MonitorContextType | undefined>(undefined);

// Create a provider component
export const MonitorProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [monitors, setMonitors] = useState<Array<{ id: string; name: string; status: string }>>([]);

  const addMonitor = (monitor: { id: string; name: string; status: string }) => {
    setMonitors((prevMonitors) => [...prevMonitors, monitor]);
  };

  const removeMonitor = (id: string) => {
    setMonitors((prevMonitors) => prevMonitors.filter(monitor => monitor.id !== id));
  };

  return (
    <MonitorContext.Provider value={{ monitors, addMonitor, removeMonitor }}>
      {children}