import React from 'react';

interface ComposeActionsProps {
  onAddCompose: () => void;
  onRemoveCompose: (id: string) => void;
  onUpdateCompose: (id: string) => void;
}

const ComposeActions: React.FC<ComposeActionsProps> = ({ onAddCompose, onRemoveCompose, onUpdateCompose }) => {
  return (
    <div>
      <h2>Compose Actions</h2>
      <button onClick={onAddCompose}>Add Compose</button>
      <button onClick={() => onRemoveCompose('example-id')}>Remove Compose</button>
      <button onClick={() => onUpdateCompose('example-id')}>Update Compose</button>
    </div>
  );
};

export default ComposeActions;