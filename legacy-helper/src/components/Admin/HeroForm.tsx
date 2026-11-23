import React, { useState, useEffect } from 'react';
import { Hero, Mood } from '../../types';
import { apiService } from '../../services/api';
import { Modal } from './Modal';
import { Save, Loader2, User, Sword, Target, FileText, Star, Zap, Shield, AlertCircle } from 'lucide-react';

interface HeroFormProps {
  hero: Hero | null;
  isOpen: boolean;
  onClose: () => void;
  onSave: () => void;
  password?: string;
}

const allMoods: Mood[] = ['aggressive', 'defensive', 'experimental', 'creative', 'chaos'];
const roles = ['Carry', 'Support', 'Mid', 'Initiator', 'Disabler', 'Tank', 'Nuker'];
const difficulties = ['Easy', 'Medium', 'Hard'];

const moodIcons: Record<Mood, React.ReactNode> = {
  aggressive: <Sword size={16} />,
  defensive: <Shield size={16} />,
  experimental: <Target size={16} />,
  creative: <Star size={16} />,
  chaos: <Zap size={16} />
};

const moodColors: Record<Mood, string> = {
  aggressive: 'text-red-500 border-red-500/30 bg-red-500/10',
  defensive: 'text-blue-500 border-blue-500/30 bg-blue-500/10',
  experimental: 'text-purple-500 border-purple-500/30 bg-purple-500/10',
  creative: 'text-yellow-500 border-yellow-500/30 bg-yellow-500/10',
  chaos: 'text-orange-500 border-orange-500/30 bg-orange-500/10'
};

export const HeroForm: React.FC<HeroFormProps> = ({ 
  hero, 
  isOpen, 
  onClose, 
  onSave, 
  password = 'ilhomcha' 
}) => {
  const [formData, setFormData] = useState<Partial<Hero>>({});
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [isSaving, setIsSaving] = useState(false);

  // Initialize form data
  useEffect(() => {
    if (hero) {
      setFormData({ ...hero });
    } else {
      setFormData({
        id: '',
        name: '',
        role: 'Carry',
        difficulty: 'Medium',
        description: '',
        moods: ['aggressive'],
        strengths: ['', '', ''],
        weaknesses: ['', '', '']
      });
    }
    setErrors({});
  }, [hero, isOpen]);

  // Validation
  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!formData.id?.trim()) {
      newErrors.id = 'Hero ID is required';
    } else if (!/^[a-z0-9-_]+$/.test(formData.id)) {
      newErrors.id = 'Hero ID must contain only lowercase letters, numbers, hyphens, and underscores';
    }

    if (!formData.name?.trim()) {
      newErrors.name = 'Hero name is required';
    }

    if (!formData.description?.trim()) {
      newErrors.description = 'Description is required';
    }

    if (!formData.moods?.length) {
      newErrors.moods = 'At least one mood must be selected';
    }

    const validStrengths = formData.strengths?.filter(s => s.trim()) || [];
    if (validStrengths.length === 0) {
      newErrors.strengths = 'At least one strength is required';
    }

    const validWeaknesses = formData.weaknesses?.filter(w => w.trim()) || [];
    if (validWeaknesses.length === 0) {
      newErrors.weaknesses = 'At least one weakness is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    
    // Clear error when user starts typing
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: '' }));
    }
  };

  const handleArrayChange = (e: React.ChangeEvent<HTMLInputElement>, index: number, field: 'strengths' | 'weaknesses') => {
    const value = e.target.value;
    setFormData(prev => {
      const newArray = [...(prev[field] || [])];
      newArray[index] = value;
      return { ...prev, [field]: newArray };
    });
    
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const addArrayItem = (field: 'strengths' | 'weaknesses') => {
    setFormData(prev => ({
      ...prev,
      [field]: [...(prev[field] || []), '']
    }));
  };

  const removeArrayItem = (index: number, field: 'strengths' | 'weaknesses') => {
    setFormData(prev => ({
      ...prev,
      [field]: (prev[field] || []).filter((_, i) => i !== index)
    }));
  };

  const handleMoodChange = (mood: Mood) => {
    setFormData(prev => {
      const currentMoods = prev.moods || [];
      const newMoods = currentMoods.includes(mood)
        ? currentMoods.filter(m => m !== mood)
        : [...currentMoods, mood];
      return { ...prev, moods: newMoods };
    });
    
    // Clear error when user makes selection
    if (errors.moods) {
      setErrors(prev => ({ ...prev, moods: '' }));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) {
      return;
    }
    setIsSaving(true);
    try {
      const finalData = {
        ...formData,
        strengths: (formData.strengths || []).filter(s => s.trim()),
        weaknesses: (formData.weaknesses || []).filter(w => w.trim())
      } as Hero;
      if (hero) {
        await apiService.updateHero(hero.id, finalData, password);
      } else {
        await apiService.createHero(finalData, password);
      }
      onSave();
    } catch (err) {
      setErrors({
        submit: err instanceof Error ? err.message : 'Failed to save hero. Please try again.'
      });
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Modal 
      isOpen={isOpen} 
      onClose={onClose} 
      title={hero ? 'Edit Hero' : 'Add New Hero'}
      size="xl"
    >
      <form onSubmit={handleSubmit} className="space-y-6">
        {/* Basic Information */}
        <div className="form-section">
          <div className="form-section-header">
            <User size={20} className="text-primary" />
            <h3 className="form-section-title">Basic Information</h3>
          </div>
          
          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="id" className="form-label">
                Hero ID *
              </label>
              <input 
                type="text" 
                id="id" 
                name="id" 
                value={formData.id || ''} 
                onChange={handleChange} 
                disabled={!!hero}
                placeholder="e.g., pudge, invoker, crystal-maiden"
                className={`form-input ${errors.id ? 'form-input-error' : ''}`}
              />
              {errors.id && (
                <div className="form-error">
                  <AlertCircle size={14} />
                  {errors.id}
                </div>
              )}
            </div>

            <div className="form-group">
              <label htmlFor="name" className="form-label">
                Hero Name *
              </label>
              <input 
                type="text" 
                id="name" 
                name="name" 
                value={formData.name || ''} 
                onChange={handleChange}
                placeholder="e.g., Pudge, Invoker, Crystal Maiden"
                className={`form-input ${errors.name ? 'form-input-error' : ''}`}
              />
              {errors.name && (
                <div className="form-error">
                  <AlertCircle size={14} />
                  {errors.name}
                </div>
              )}
            </div>

            <div className="form-group">
              <label htmlFor="role" className="form-label">
                Primary Role
              </label>
              <select 
                id="role" 
                name="role" 
                value={formData.role || ''} 
                onChange={handleChange}
                className="form-select"
              >
                {roles.map(role => (
                  <option key={role} value={role}>{role}</option>
                ))}
              </select>
            </div>

            <div className="form-group">
              <label htmlFor="difficulty" className="form-label">
                Difficulty
              </label>
              <select 
                id="difficulty" 
                name="difficulty" 
                value={formData.difficulty || ''} 
                onChange={handleChange}
                className="form-select"
              >
                {difficulties.map(diff => (
                  <option key={diff} value={diff}>{diff}</option>
                ))}
              </select>
            </div>

            <div className="form-group full-width">
              <label htmlFor="description" className="form-label">
                Description *
              </label>
              <textarea 
                id="description" 
                name="description" 
                value={formData.description || ''} 
                onChange={handleChange}
                rows={3}
                placeholder="Brief description of the hero's role and capabilities..."
                className={`form-textarea ${errors.description ? 'form-input-error' : ''}`}
              />
              {errors.description && (
                <div className="form-error">
                  <AlertCircle size={14} />
                  {errors.description}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Moods */}
        <div className="form-section">
          <div className="form-section-header">
            <Zap size={20} className="text-primary" />
            <h3 className="form-section-title">Moods *</h3>
          </div>
          
          <div className="mood-grid">
            {allMoods.map(mood => (
              <button
                key={mood}
                type="button"
                onClick={() => handleMoodChange(mood)}
                className={`mood-button ${
                  formData.moods?.includes(mood) 
                    ? `mood-button-active ${moodColors[mood]}` 
                    : 'mood-button-inactive'
                }`}
              >
                {moodIcons[mood]}
                <span className="capitalize">{mood}</span>
              </button>
            ))}
          </div>
          {errors.moods && (
            <div className="form-error">
              <AlertCircle size={14} />
              {errors.moods}
            </div>
          )}
        </div>

        {/* Strengths */}
        <div className="form-section">
          <div className="form-section-header">
            <Star size={20} className="text-green-500" />
            <h3 className="form-section-title">Strengths *</h3>
            <button
              type="button"
              onClick={() => addArrayItem('strengths')}
              className="btn-sm btn-outline"
            >
              Add Strength
            </button>
          </div>
          
          <div className="array-inputs">
            {(formData.strengths || []).map((strength, index) => (
              <div key={index} className="array-input-group">
                <input
                  type="text"
                  value={strength}
                  onChange={(e) => handleArrayChange(e, index, 'strengths')}
                  placeholder={`Strength ${index + 1}`}
                  className="form-input"
                />
                {(formData.strengths?.length || 0) > 1 && (
                  <button
                    type="button"
                    onClick={() => removeArrayItem(index, 'strengths')}
                    className="btn-icon btn-danger"
                  >
                    ×
                  </button>
                )}
              </div>
            ))}
          </div>
          {errors.strengths && (
            <div className="form-error">
              <AlertCircle size={14} />
              {errors.strengths}
            </div>
          )}
        </div>

        {/* Weaknesses */}
        <div className="form-section">
          <div className="form-section-header">
            <Target size={20} className="text-red-500" />
            <h3 className="form-section-title">Weaknesses *</h3>
            <button
              type="button"
              onClick={() => addArrayItem('weaknesses')}
              className="btn-sm btn-outline"
            >
              Add Weakness
            </button>
          </div>
          
          <div className="array-inputs">
            {(formData.weaknesses || []).map((weakness, index) => (
              <div key={index} className="array-input-group">
                <input
                  type="text"
                  value={weakness}
                  onChange={(e) => handleArrayChange(e, index, 'weaknesses')}
                  placeholder={`Weakness ${index + 1}`}
                  className="form-input"
                />
                {(formData.weaknesses?.length || 0) > 1 && (
                  <button
                    type="button"
                    onClick={() => removeArrayItem(index, 'weaknesses')}
                    className="btn-icon btn-danger"
                  >
                    ×
                  </button>
                )}
              </div>
            ))}
          </div>
          {errors.weaknesses && (
            <div className="form-error">
              <AlertCircle size={14} />
              {errors.weaknesses}
            </div>
          )}
        </div>

        {/* Submit Error */}
        {errors.submit && (
          <div className="form-error text-center p-4 bg-red-50 rounded-lg border border-red-200">
            <AlertCircle size={16} />
            {errors.submit}
          </div>
        )}

        {/* Form Actions */}
        <div className="form-actions">
          <button 
            type="button" 
            onClick={onClose} 
            className="btn btn-secondary"
            disabled={isSaving}
          >
            Cancel
          </button>
          <button 
            type="submit" 
            className="btn btn-primary"
            disabled={isSaving}
          >
            {isSaving ? (
              <>
                <Loader2 size={16} className="animate-spin" />
                Saving...
              </>
            ) : (
              <>
                <Save size={16} />
                {hero ? 'Update Hero' : 'Create Hero'}
              </>
            )}
          </button>
        </div>
      </form>
    </Modal>
  );
}; 