import React from 'react';

interface ImportActionsProps {
  onAddImport: () => void;
  onRemoveImport: (id: string) => void;
  onUpdateImport: (id: string) => void;
}

const ImportActions: React.FC<ImportActionsProps> = ({ onAddImport, onRemoveImport, onUpdateImport }) => {
  return (
    <div>
      <h2>Import Actions</h2>
      <button onClick={onAddImport}>Add Import</button>
      <button onClick={() => onRemoveImport('example-id')}>Remove Import</button>
      <button onClick={() => onUpdateImport('example-id')}>Update Import</button>
    </div>
  );
};

export default ImportActions;