import { Build } from '../../types';

interface BuildFormProps {
  build?: Build;
  heroId?: number;
  isOpen: boolean;
  onClose: () => void;
  onSave: (build: Build) => void;
  password: string;
}

export const BuildForm = ({ build, heroId, isOpen, onClose, onSave, password }: BuildFormProps) => {
  // TODO: Add state, handlers, and return JSX here
  return (
    <div>
      {/* TODO: Add MUI Dialog and form JSX here */}
    </div>
  );
};

