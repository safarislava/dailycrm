export type Role = 'gip' | 'lawyer' | 'accountant'

export interface Project {
  id: string
  title: string
  updated_at: string
}

export interface Stage {
  project_id: string
  position: number
  title: string
  deadline: string | null
  completed: boolean
}

export interface DetailedStage {
  project_id: string
  position: number
  title: string
  deadline: string | null
  completed: boolean
  cost: number | null
  gip_confirmed: boolean
  payment_confirmed: boolean
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