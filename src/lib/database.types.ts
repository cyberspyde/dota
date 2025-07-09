export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

export interface Database {
  public: {
    Tables: {
      heroes: {
        Row: {
          id: string
          name: string
          role: string
          difficulty: string
          description: string | null
          created_at: string
          updated_at: string
        }
        Insert: {
          id: string
          name: string
          role: string
          difficulty: string
          description?: string | null
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          name?: string
          role?: string
          difficulty?: string
          description?: string | null
          created_at?: string
          updated_at?: string
        }
      }
      hero_moods: {
        Row: {
          hero_id: string
          mood: string
        }
        Insert: {
          hero_id: string
          mood: string
        }
        Update: {
          hero_id?: string
          mood?: string
        }
      }
      hero_strengths: {
        Row: {
          id: number
          hero_id: string
          strength: string
          order_index: number
        }
        Insert: {
          id?: number
          hero_id: string
          strength: string
          order_index?: number
        }
        Update: {
          id?: number
          hero_id?: string
          strength?: string
          order_index?: number
        }
      }
      hero_weaknesses: {
        Row: {
          id: number
          hero_id: string
          weakness: string
          order_index: number
        }
        Insert: {
          id?: number
          hero_id: string
          weakness: string
          order_index?: number
        }
        Update: {
          id?: number
          hero_id?: string
          weakness?: string
          order_index?: number
        }
      }
      builds: {
        Row: {
          id: number
          hero_id: string
          mood: string
          early_game: string | null
          mid_game: string | null
          late_game: string | null
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: number
          hero_id: string
          mood: string
          early_game?: string | null
          mid_game?: string | null
          late_game?: string | null
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: number
          hero_id?: string
          mood?: string
          early_game?: string | null
          mid_game?: string | null
          late_game?: string | null
          created_at?: string
          updated_at?: string
        }
      }
      items: {
        Row: {
          id: number
          build_id: number
          name: string
          cost: number
          phase: string
          priority: string
          description: string | null
          order_index: number
        }
        Insert: {
          id?: number
          build_id: number
          name: string
          cost: number
          phase: string
          priority: string
          description?: string | null
          order_index?: number
        }
        Update: {
          id?: number
          build_id?: number
          name?: string
          cost?: number
          phase?: string
          priority?: string
          description?: string | null
          order_index?: number
        }
      }
      playstyle_dos: {
        Row: {
          id: number
          build_id: number
          do_item: string
          order_index: number
        }
        Insert: {
          id?: number
          build_id: number
          do_item: string
          order_index?: number
        }
        Update: {
          id?: number
          build_id?: number
          do_item?: string
          order_index?: number
        }
      }
      playstyle_donts: {
        Row: {
          id: number
          build_id: number
          dont_item: string
          order_index: number
        }
        Insert: {
          id?: number
          build_id: number
          dont_item: string
          order_index?: number
        }
        Update: {
          id?: number
          build_id?: number
          dont_item?: string
          order_index?: number
        }
      }
      playstyle_tips: {
        Row: {
          id: number
          build_id: number
          tip: string
          order_index: number
        }
        Insert: {
          id?: number
          build_id: number
          tip: string
          order_index?: number
        }
        Update: {
          id?: number
          build_id?: number
          tip?: string
          order_index?: number
        }
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}