import { Hero, Build } from '../types';

class ApiService {
  private baseUrl: string;

  constructor() {
    // Use a relative path for API calls. This is more robust for local development
    // and works seamlessly in production.
    this.baseUrl = '/api';
  }

  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    console.log(`Making request to: ${url}`);
    
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
      ...options,
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error(`API Error: ${response.status} - ${errorText}`);
      throw new Error(`API Error: ${response.status} - ${response.statusText}`);
    }

    return response.json();
  }

  async getHeroes(limit: number, offset: number, mood?: string): Promise<{ heroes: Hero[], total: number }> {
    const params = new URLSearchParams({
      limit: String(limit),
      offset: String(offset),
    });
    if (mood) {
      params.append('mood', mood);
    }
    return this.request<{ heroes: Hero[], total: number }>(`/heroes?${params.toString()}`);
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

  async createHero(heroData: Hero, token: string): Promise<Hero> {
    return this.request<Hero>('/heroes', {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${token}` },
      body: JSON.stringify(heroData),
    });
  }

  async updateHero(id: string, heroData: Partial<Hero>, token: string): Promise<Hero> {
    return this.request<Hero>(`/heroes/${id}`, {
      method: 'PUT',
      headers: { 'Authorization': `Bearer ${token}` },
      body: JSON.stringify(heroData),
    });
  }

  async deleteHero(id: string, token: string): Promise<void> {
    return this.request<void>(`/heroes/${id}`, {
      method: 'DELETE',
      headers: { 'Authorization': `Bearer ${token}` },
    });
  }

  async createBuild(buildData: Build, token: string): Promise<Build> {
    return this.request<Build>('/builds', {
      method: 'POST',
      headers: { 
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(buildData),
    });
  }

  async updateBuild(id: number, buildData: Partial<Build>, token: string): Promise<Build> {
    return this.request<Build>(`/builds/${id}`, {
      method: 'PUT',
      headers: { 
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(buildData),
    });
  }

  async deleteBuild(id: number, token: string): Promise<void> {
    return this.request<void>(`/builds/${id}`, {
      method: 'DELETE',
      headers: { 'Authorization': `Bearer ${token}` },
    });
  }

  async checkHealth(): Promise<{ status: string; timestamp: string }> {
    return this.request<{ status: string; timestamp: string }>('/health');
  }
}

export const apiService = new ApiService();