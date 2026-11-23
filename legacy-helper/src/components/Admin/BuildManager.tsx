import React, { useState, useEffect } from 'react';
import { Hero, Build } from '../../types';
import { apiService } from '../../services/api';
import { BuildForm } from './BuildForm';
import { Plus, Edit, Trash2 } from 'lucide-react';

export const BuildManager: React.FC = () => {
  const [heroes, setHeroes] = useState<Hero[]>([]);
  const [selectedHeroId, setSelectedHeroId] = useState<string>('');
  const [builds, setBuilds] = useState<Build[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedBuild, setSelectedBuild] = useState<Build | null>(null);

  useEffect(() => {
    const fetchHeroes = async () => {
      try {
        const result = await apiService.getHeroes(1000, 0);
        setHeroes(result.heroes);
        if (result.heroes.length > 0) {
          setSelectedHeroId(result.heroes[0].id);
        }
      } catch (err) {
        setError('Failed to fetch heroes.');
      } finally {
        setIsLoading(false);
      }
    };
    fetchHeroes();
  }, []);

  useEffect(() => {
    if (!selectedHeroId) return;
    const fetchBuilds = async () => {
      try {
        setIsLoading(true);
        const heroBuilds = await apiService.getHeroBuilds(selectedHeroId);
        setBuilds(heroBuilds);
      } catch (err) {
        setError('Failed to fetch builds.');
      } finally {
        setIsLoading(false);
      }
    };
    fetchBuilds();
  }, [selectedHeroId]);

  const handleOpenModal = (build: Build | null = null) => {
    setSelectedBuild(build);
    setIsModalOpen(true);
  };
  
  const handleCloseModal = () => {
    setIsModalOpen(false);
    setSelectedBuild(null);
  };

  const handleSave = async () => {
    if (selectedHeroId) {
      const heroBuilds = await apiService.getHeroBuilds(selectedHeroId);
      setBuilds(heroBuilds);
    }
    handleCloseModal();
  };

  const handleDelete = async (id: number) => {
    if (window.confirm('Are you sure you want to delete this build?')) {
      try {
        await apiService.deleteBuild(id, 'ilhomcha');
        if (selectedHeroId) {
          const heroBuilds = await apiService.getHeroBuilds(selectedHeroId);
          setBuilds(heroBuilds);
        }
      } catch (err) {
        setError('Failed to delete build.');
      }
    }
  };

  if (isLoading && heroes.length === 0) return <div>Loading...</div>;
  if (error) return <div className="error-message">{error}</div>;

  return (
    <div className="card">
      <div className="admin-section-header">
        <h2 className="admin-section-title">Build Management</h2>
        <div className="admin-controls">
          <select value={selectedHeroId} onChange={e => setSelectedHeroId(e.target.value)}>
            {heroes.map(hero => (
              <option key={hero.id} value={hero.id}>{hero.name}</option>
            ))}
          </select>
          <button onClick={() => handleOpenModal()} className="btn btn-primary" disabled={!selectedHeroId}>
            <Plus size={16} />
            <span>Add New Build</span>
          </button>
        </div>
      </div>
      
      <div className="admin-table-container">
        <table className="admin-table">
          <thead>
            <tr>
              <th>Mood</th>
              <th>Items</th>
              <th>Score</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {builds.map(build => (
              <tr key={`${build.heroId}-${build.mood}`}>
                <td>{build.mood}</td>
                <td>{build.items.length}</td>
                <td>{build.score?.toFixed(1) || 'N/A'}</td>
                <td>
                  <button onClick={() => handleOpenModal(build)} className="btn-icon"><Edit size={16} /></button>
                  <button onClick={() => handleDelete(build.id as any)} className="btn-icon btn-danger"><Trash2 size={16} /></button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <BuildForm
        build={selectedBuild}
        heroId={selectedHeroId}
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        onSave={handleSave}
        password="ilhomcha"
      />
    </div>
  );
}; 