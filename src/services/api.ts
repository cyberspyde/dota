import { Hero, Build } from '../types';

class ApiService {
  private baseUrl: string;

  constructor() {
    // Use a relative path for API calls. This is more robust for local development
    // and works seamlessly in production.
    this.baseUrl = '/api';
  }

  private async request<T>(endpoint: string): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    console.log(`Making request to: ${url}`);
    
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error(`API Error: ${response.status} - ${errorText}`);
      throw new Error(`API Error: ${response.status} - ${response.statusText}`);
    }

    return response.json();
  }

  async getHeroes(): Promise<Hero[]> {
    return this.request<Hero[]>('/heroes');
  }

  async getHero(id: string): Promise<Hero> {
    return this.request<Hero>(`/heroes/${id}`);
  }

  async getBuilds(): Promise<Build[]> {
    return this.request<Build[]>('/builds');
  }

  async getHeroBuilds(heroId: string): Promise<Build[]> {
    return this.request<Build[]>(`/heroes/${heroId}/builds`);
  }

  async getBuild(heroId: string, mood: string): Promise<Build> {
    return this.request<Build>(`/heroes/${heroId}/builds/${mood}`);
  }

  async checkHealth(): Promise<{ status: string; timestamp: string }> {
    return this.request<{ status: string; timestamp: string }>('/health');
  }
}

export const apiService = new ApiService();