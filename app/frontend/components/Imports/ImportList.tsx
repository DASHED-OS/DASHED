import React from 'react';
import { useImports } from './ImportContext';
import ImportItem from './importItem';

const ImportList: React.FC = () => {
  const { imports } = useImports();

  return (
    <div>
      <h2>Import List</h2>
      {imports.length > 0 ? (
        imports.map((importItem, index) => (
          <ImportItem
            key={index}
            id={importItem.id}
            name={importItem.name}
            description={importItem.description}
            onSelect={(id) => console.log(`Selected import with id: ${id}`)}
          />
        ))
      ) : (
        <p>No imports available.</p>
      )}
    </div>
  );
};

export default ImportList;