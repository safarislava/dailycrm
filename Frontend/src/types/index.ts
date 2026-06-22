export type Role = 'gip' | 'lawyer' | 'accountant'

export interface Project {
  id: string
  title: string
  updated_at: string
}

export interface Stage {
  project_id: string
  parent_position: number
  position: number
  title: string
  deadline: string | null
  completed: boolean
}

export interface DetailedStage {
  project_id: string
  parent_position: number
  position: number
  title: string
  deadline: string | null
  completed: boolean
  advance_cost: number | null
  advance_confirmed: boolean
  final_cost: number | null
  final_confirmed: boolean
  gip_confirmed: boolean
}

export type Act = Attachment

export interface StageWithProjectTitle {
  stage: Stage
  project_title: string
}

export interface Attachment {
  id: string
  filename: string
  mime_type: string
  size_bytes: number
  created_at: string
  download_url: string
}

export interface Comment {
  id: string
  text: string
  author: string
  is_system: boolean
  created_at: string
}