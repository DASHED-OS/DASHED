import React from 'react';

interface ExportActionsProps {
  onAddExport: () => void;
  onRemoveExport: (id: string) => void;
  onUpdateExport: (id: string) => void;
}

const ExportActions: React.FC<ExportActionsProps> = ({ onAddExport, onRemoveExport, onUpdateExport }) => {
  return (
    <div>
      <h2>Export Actions</h2>
      <button onClick={onAddExport}>Add Export</button>
      <button onClick={() => onRemoveExport('example-id')}>Remove Export</button>
      <button onClick={() => onUpdateExport('example-id')}>Update Export</button>
    </div>
  );
};

export default ExportActions;