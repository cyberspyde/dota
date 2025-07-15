import React, { useState, useEffect } from 'react';
import { Hero } from '../../types';
import { apiService } from '../../services/api';
import { HeroForm } from './HeroForm';
import { Plus, Edit, Trash2 } from 'lucide-react';

export const HeroManager: React.FC = () => {
  const [heroes, setHeroes] = useState<Hero[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedHero, setSelectedHero] = useState<Hero | null>(null);

  const fetchHeroes = async () => {
    try {
      setIsLoading(true);
      // Fetch all heroes for the admin panel
      const result = await apiService.getHeroes(1000, 0); 
      setHeroes(result.heroes);
    } catch (err) {
      setError('Failed to fetch heroes.');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchHeroes();
  }, []);

  const handleOpenModal = (hero: Hero | null = null) => {
    setSelectedHero(hero);
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setIsModalOpen(false);
    setSelectedHero(null);
  };

  const handleSave = async () => {
    await fetchHeroes();
    handleCloseModal();
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this hero? This action cannot be undone.')) {
      try {
        await apiService.deleteHero(id, 'ilhomcha'); // Using hardcoded password for now
        await fetchHeroes();
      } catch (err) {
        setError('Failed to delete hero.');
      }
    }
  };

  if (isLoading) return <div>Loading heroes...</div>;
  if (error) return <div className="error-message">{error}</div>;

  return (
    <div className="card">
      <div className="admin-section-header">
        <h2 className="admin-section-title">Hero Management</h2>
        <button onClick={() => handleOpenModal()} className="btn btn-primary">
          <Plus size={16} />
          <span>Add New Hero</span>
        </button>
      </div>

      <div className="admin-table-container">
        <table className="admin-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Role</th>
              <th>Difficulty</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {heroes.map(hero => (
              <tr key={hero.id}>
                <td>{hero.name}</td>
                <td>{hero.role}</td>
                <td>{hero.difficulty}</td>
                <td>
                  <button onClick={() => handleOpenModal(hero)} className="btn-icon">
                    <Edit size={16} />
                  </button>
                  <button onClick={() => handleDelete(hero.id)} className="btn-icon btn-danger">
                    <Trash2 size={16} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {isModalOpen && (
        <HeroForm
          hero={selectedHero}
          onClose={handleCloseModal}
          onSave={handleSave}
          password="ilhomcha" // Using hardcoded password for now
        />
      )}
    </div>
  );
}; 