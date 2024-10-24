import React, { useState, ReactNode } from 'react';
import { DashboardContext } from './DashboardContext';

interface DashboardProviderProps {
  children: ReactNode;
}

interface Widget {
  id: string;
  name: string;
  data: any;
}

const DashboardProvider: React.FC<DashboardProviderProps> = ({ children }) => {
  const [widgets, setWidgets] = useState<Widget[]>([]);

  const addWidget = (widget: Widget) => {
    setWidgets((prevWidgets) => [...prevWidgets, widget]);
  };

  const removeWidget = (id: string) => {
    setWidgets((prevWidgets) => prevWidgets.filter(widget => widget.id !== id));
  };

  return (
    <DashboardContext.Provider value={{ widgets, addWidget, removeWidget }}>
      {children}
    </DashboardContext.Provider>
  );
};

export default DashboardProvider;