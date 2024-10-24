import React from 'react';

interface Widget {
  id: string;
  name: string;
  content: React.ReactNode;
}

interface DashboardWidgetsProps {
  widgets: Widget[];
}

const DashboardWidgets: React.FC<DashboardWidgetsProps> = ({ widgets }) => {
  return (
    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '20px' }}>
      {widgets.map((widget) => (
        <div key={widget.id} style={{ border: '1px solid #ccc', padding: '10px', borderRadius: '5px', flex: '1 1 calc(33.333% - 20px)' }}>
          <h3>{widget.name}</h3>
          <div>{widget.content}</div>
        </div>
      ))}
    </div>
  );
};

export default DashboardWidgets;