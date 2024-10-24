import React from 'react';

interface ExportLogsProps {
  logs: string[];
}

const ExportLogs: React.FC<ExportLogsProps> = ({ logs }) => {
  return (
    <div>
      <h2>Export Logs</h2>
      {logs.length > 0 ? (
        <ul>
          {logs.map((log, index) => (
            <li key={index}>{log}</li>
          ))}
        </ul>
      ) : (
        <p>No logs available.</p>
      )}
    </div>
  );
};

export default ExportLogs;