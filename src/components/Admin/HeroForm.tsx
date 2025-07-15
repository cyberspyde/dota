import React, { useState, useEffect } from 'react';
import { Hero, Mood } from '../../types';
import { apiService } from '../../services/api';

interface HeroFormProps {
  hero: Hero | null;
  onClose: () => void;
  onSave: () => void;
  password?: string;
}

const allMoods: Mood[] = ['aggressive', 'defensive', 'experimental', 'creative', 'chaos'];

export const HeroForm: React.FC<HeroFormProps> = ({ hero, onClose, onSave, password }) => {
  const [formData, setFormData] = useState<Partial<Hero>>({});
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);

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
        moods: ['aggressive'], // Default mood
        strengths: ['', '', ''],
        weaknesses: ['', '', '']
      });
    }
  }, [hero]);
  
  // Ensure arrays are always initialized
  useEffect(() => {
    setFormData(prev => ({
      ...prev,
      moods: prev.moods || [],
      strengths: prev.strengths || ['', '', ''],
      weaknesses: prev.weaknesses || ['', '', '']
    }));
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleArrayChange = (e: React.ChangeEvent<HTMLInputElement>, index: number, field: 'strengths' | 'weaknesses') => {
    const newValue = [...(formData[field] || [])];
    newValue[index] = e.target.value;
    setFormData(prev => ({ ...prev, [field]: newValue }));
  };

  const handleMoodChange = (mood: Mood) => {
    const currentMoods = formData.moods || [];
    const newMoods = currentMoods.includes(mood)
      ? currentMoods.filter(m => m !== mood)
      : [...currentMoods, mood];
    setFormData(prev => ({ ...prev, moods: newMoods }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!password) {
      setError("Authentication token is missing.");
      return;
    }
    
    setIsSaving(true);
    setError(null);
    
    try {
      const finalData = {
        ...formData,
        strengths: formData.strengths?.filter(s => s.trim() !== ''),
        weaknesses: formData.weaknesses?.filter(w => w.trim() !== ''),
      } as Hero;

      if (hero) {
        await apiService.updateHero(hero.id, finalData, password);
      } else {
        await apiService.createHero(finalData, password);
      }
      onSave();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unknown error occurred.');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="modal-backdrop">
      <div className="modal-content card">
        <h2 className="modal-title">{hero ? 'Edit Hero' : 'Add New Hero'}</h2>
        <form onSubmit={handleSubmit} className="admin-form">
          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="id">Hero ID</label>
              <input type="text" id="id" name="id" value={formData.id || ''} onChange={handleChange} required disabled={!!hero} />
            </div>
            <div className="form-group">
              <label htmlFor="name">Hero Name</label>
              <input type="text" id="name" name="name" value={formData.name || ''} onChange={handleChange} required />
            </div>
            <div className="form-group">
              <label htmlFor="role">Role</label>
              <select id="role" name="role" value={formData.role || ''} onChange={handleChange}>
                <option value="Carry">Carry</option>
                <option value="Support">Support</option>
                <option value="Mid">Mid</option>
                <option value="Initiator">Initiator</option>
              </select>
            </div>
            <div className="form-group">
              <label htmlFor="difficulty">Difficulty</label>
              <select id="difficulty" name="difficulty" value={formData.difficulty || ''} onChange={handleChange}>
                <option value="Easy">Easy</option>
                <option value="Medium">Medium</option>
                <option value="Hard">Hard</option>
              </select>
            </div>
            <div className="form-group full-width">
              <label htmlFor="description">Description</label>
              <textarea id="description" name="description" value={formData.description || ''} onChange={handleChange} rows={2}></textarea>
            </div>
            <div className="form-group full-width">
              <label>Moods</label>
              <div className="mood-checkbox-group">
                {allMoods.map(mood => (
                  <label key={mood} className="mood-checkbox">
                    <input
                      type="checkbox"
                      checked={formData.moods?.includes(mood)}
                      onChange={() => handleMoodChange(mood)}
                    />
                    <span>{mood}</span>
                  </label>
                ))}
              </div>
            </div>
            <div className="form-group">
              <label>Strengths</label>
              {formData.strengths?.map((strength, index) => (
                <input key={index} type="text" value={strength} onChange={(e) => handleArrayChange(e, index, 'strengths')} />
              ))}
            </div>
            <div className="form-group">
              <label>Weaknesses</label>
              {formData.weaknesses?.map((weakness, index) => (
                <input key={index} type="text" value={weakness} onChange={(e) => handleArrayChange(e, index, 'weaknesses')} />
              ))}
            </div>
          </div>
          {error && <div className="error-message">{error}</div>}
          <div className="form-actions">
            <button type="button" onClick={onClose} className="btn btn-secondary" disabled={isSaving}>
              Cancel
            </button>
            <button type="submit" className="btn btn-primary" disabled={isSaving}>
              {isSaving ? 'Saving...' : 'Save Hero'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}; 