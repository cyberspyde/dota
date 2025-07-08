const API_BASE_URL = 'http://localhost:3001/api';

export interface Hero {
  id: string;
  name: string;
  role: string;
  difficulty: 'Easy' | 'Medium' | 'Hard';
  description: string;
  moods: string[];
  strengths: string[];
  weaknesses: string[];
}

export interface Build {
  heroId: string;
  mood: string;
  items: {
    id: number;
    name: string;
    cost: number;
    phase: 'Early' | 'Mid' | 'Late';
    priority: 'Core' | 'Situational' | 'Luxury';
    description: string;
  }[];
  playstyle: {
    dos: string[];
    donts: string[];
    tips: string[];
  };
  gameplan: {
    early: string;
    mid: string;
    late: string;
  };
}

class ApiService {
  private async fetchWithErrorHandling<T>(url: string): Promise<T> {
    try {
      const response = await fetch(url);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error(`API Error (${url}):`, error);
      throw new Error(`Failed to fetch data from ${url}`);
    }
  }

  async getHeroes(): Promise<Hero[]> {
    return this.fetchWithErrorHandling<Hero[]>(`${API_BASE_URL}/heroes`);
  }

  async getHero(id: string): Promise<Hero> {
    return this.fetchWithErrorHandling<Hero>(`${API_BASE_URL}/heroes/${id}`);
  }

  async getBuilds(): Promise<Build[]> {
    return this.fetchWithErrorHandling<Build[]>(`${API_BASE_URL}/builds`);
  }

  async getHeroBuilds(heroId: string): Promise<Build[]> {
    return this.fetchWithErrorHandling<Build[]>(`${API_BASE_URL}/heroes/${heroId}/builds`);
  }

  async getBuild(heroId: string, mood: string): Promise<Build> {
    return this.fetchWithErrorHandling<Build>(`${API_BASE_URL}/heroes/${heroId}/builds/${mood}`);
  }

  async checkHealth(): Promise<{ status: string; timestamp: string }> {
    return this.fetchWithErrorHandling<{ status: string; timestamp: string }>(`${API_BASE_URL}/health`);
  }
}

export const apiService = new ApiService();