import React from 'react';
import { useExports } from './ExportContext';
import ExportItem from './ExportItem';

const ExportList: React.FC = () => {
  const { exports } = useExports();

  return (
    <div>
      <h2>Export List</h2>
      {exports.length > 0 ? (
        exports.map((exportItem, index) => (
          <ExportItem
            key={index}
            id={exportItem.id}
            name={exportItem.name}
            description={exportItem.description}
            onSelect={(id) => console.log(`Selected export with id: ${id}`)}
          />
        ))
      ) : (
        <p>No exports available.</p>
      )}
    </div>
  );
};

export default ExportList;