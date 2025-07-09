import { Hero, Build } from '../types';

class ApiService {
  private baseUrl: string;

  constructor() {
    // Use Vercel API endpoints
    // For production, use Vercel's VERCEL_URL or custom VITE_API_URL
    this.baseUrl = process.env.NODE_ENV === 'production' 
      ? (import.meta.env.VITE_API_URL || `https://${import.meta.env.VITE_VERCEL_URL || 'your-vercel-app.vercel.app'}/api`)
      : 'http://localhost:3000/api';
  }

  private async fetchWithErrorHandling<T>(url: string): Promise<T> {
    try {
      const response = await fetch(url);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('API request failed:', error);
      throw error;
    }
  }

  async getHeroes(): Promise<Hero[]> {
    return this.fetchWithErrorHandling<Hero[]>(`${this.baseUrl}/heroes`);
  }

  async getHero(id: string): Promise<Hero> {
    return this.fetchWithErrorHandling<Hero>(`${this.baseUrl}/heroes/${id}`);
  }

  async getBuilds(): Promise<Build[]> {
    return this.fetchWithErrorHandling<Build[]>(`${this.baseUrl}/builds`);
  }

  async getHeroBuilds(heroId: string): Promise<Build[]> {
    return this.fetchWithErrorHandling<Build[]>(`${this.baseUrl}/heroes/${heroId}/builds`);
  }

  async getBuild(heroId: string, mood: string): Promise<Build> {
    return this.fetchWithErrorHandling<Build>(`${this.baseUrl}/heroes/${heroId}/builds/${mood}`);
  }

  async checkHealth(): Promise<{ status: string; timestamp: string }> {
    return this.fetchWithErrorHandling<{ status: string; timestamp: string }>(`${this.baseUrl}/health`);
  }
}

export const apiService = new ApiService();