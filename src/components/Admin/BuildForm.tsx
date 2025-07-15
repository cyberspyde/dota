import React, { useState, useEffect } from 'react';
import { Build, Mood, Item } from '../../types';
import { apiService } from '../../services/api';

interface BuildFormProps {
  build: Build | null;
  heroId: string;
  onClose: () => void;
  onSave: () => void;
  password?: string;
}

const allMoods: Mood[] = ['aggressive', 'defensive', 'experimental', 'creative', 'chaos'];
const allPhases: Item['phase'][] = ['Early', 'Mid', 'Late'];
const allPriorities: Item['priority'][] = ['Core', 'Situational', 'Luxury'];

export const BuildForm: React.FC<BuildFormProps> = ({ build, heroId, onClose, onSave, password }) => {
  const [formData, setFormData] = useState<Partial<Build>>({});
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (build) {
      setFormData({ ...build });
    } else {
      setFormData({
        heroId,
        mood: allMoods[0],
        items: Array(6).fill({ name: '', cost: 0, phase: 'Early', priority: 'Core', description: '' }),
        playstyle: { dos: Array(3).fill(''), donts: Array(3).fill(''), tips: Array(3).fill('') },
        gameplan: { early: '', mid: '', late: '' }
      });
    }
  }, [build, heroId]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    if (name.includes('.')) {
      const [field, subfield] = name.split('.');
      setFormData(prev => {
        const prevField = prev[field as keyof typeof prev];
        return {
          ...prev,
          [field]: { 
            ...(typeof prevField === 'object' ? prevField : {}), 
            [subfield]: value 
          }
        };
      });
    } else {
      setFormData(prev => ({ ...prev, [name]: value }));
    }
  };

  const handleItemChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>, index: number) => {
    const { name, value } = e.target;
    const newItems = [...(formData.items || [])];
    newItems[index] = { ...newItems[index], [name]: name === 'cost' ? parseInt(value) : value };
    setFormData(prev => ({ ...prev, items: newItems }));
  };

  const handlePlaystyleChange = (e: React.ChangeEvent<HTMLInputElement>, index: number, field: 'dos' | 'donts' | 'tips') => {
    const newArr = [...(formData.playstyle?.[field] || [])];
    newArr[index] = e.target.value;
    setFormData(prev => ({ ...prev, playstyle: { ...prev.playstyle, [field]: newArr } as any }));
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
      const finalData = { ...formData } as Build;

      if (build) {
        await apiService.updateBuild(build.id!, finalData, password);
      } else {
        await apiService.createBuild(finalData, password);
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
        <h2 className="modal-title">{build ? 'Edit Build' : 'Add New Build'}</h2>
        <form onSubmit={handleSubmit} className="admin-form">
          <div className="form-group">
            <label>Mood</label>
            <select name="mood" value={formData.mood} onChange={handleChange}>
              {allMoods.map(m => <option key={m} value={m}>{m}</option>)}
            </select>
          </div>
          
          <h3>Items</h3>
          {formData.items?.map((item, index) => (
            <div key={index} className="form-grid">
              <input name="name" value={item.name} onChange={e => handleItemChange(e, index)} placeholder="Item Name"/>
              <input name="cost" type="number" value={item.cost} onChange={e => handleItemChange(e, index)} placeholder="Cost"/>
              <select name="phase" value={item.phase} onChange={e => handleItemChange(e, index)}>
                {allPhases.map(p => <option key={p} value={p}>{p}</option>)}
              </select>
              <select name="priority" value={item.priority} onChange={e => handleItemChange(e, index)}>
                {allPriorities.map(p => <option key={p} value={p}>{p}</option>)}
              </select>
              <input name="description" value={item.description} onChange={e => handleItemChange(e, index)} placeholder="Description" className="full-width"/>
            </div>
          ))}

          <h3>Playstyle</h3>
          <div className="form-grid">
            <div>
              <label>Do's</label>
              {formData.playstyle?.dos.map((v, i) => <input key={i} value={v} onChange={e => handlePlaystyleChange(e, i, 'dos')} />)}
            </div>
            <div>
              <label>Don'ts</label>
              {formData.playstyle?.donts.map((v, i) => <input key={i} value={v} onChange={e => handlePlaystyleChange(e, i, 'donts')} />)}
            </div>
            <div>
              <label>Tips</label>
              {formData.playstyle?.tips.map((v, i) => <input key={i} value={v} onChange={e => handlePlaystyleChange(e, i, 'tips')} />)}
            </div>
          </div>
          
          <h3>Gameplan</h3>
          <div className="form-grid">
            <textarea name="gameplan.early" value={formData.gameplan?.early} onChange={handleChange} placeholder="Early Game"/>
            <textarea name="gameplan.mid" value={formData.gameplan?.mid} onChange={handleChange} placeholder="Mid Game"/>
            <textarea name="gameplan.late" value={formData.gameplan?.late} onChange={handleChange} placeholder="Late Game"/>
          </div>

          {error && <div className="error-message">{error}</div>}
          <div className="form-actions">
            <button type="button" onClick={onClose} className="btn btn-secondary" disabled={isSaving}>
              Cancel
            </button>
            <button type="submit" className="btn btn-primary" disabled={isSaving}>
              {isSaving ? 'Saving...' : 'Save Build'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}; 