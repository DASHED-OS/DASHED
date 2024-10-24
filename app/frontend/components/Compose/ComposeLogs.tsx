import React from 'react';

interface ComposeLogsProps {
  logs: string[];
}

const ComposeLogs: React.FC<ComposeLogsProps> = ({ logs }) => {
  return (
    <div>
      <h2>Compose Logs</h2>
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

export default ComposeLogs;