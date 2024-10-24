import React from 'react';

interface ImportLogsProps {
  logs: string[];
}

const ImportLogs: React.FC<ImportLogsProps> = ({ logs }) => {
  return (
    <div>
      <h2>Import Logs</h2>
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

export default ImportLogs;