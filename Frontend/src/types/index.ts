export type Role = 'gip' | 'lawyer' | 'accountant' | 'admin'

export interface Project {
  id: string
  title: string
  updated_at: string
}

export interface StagePaymentTranche {
  id: string
  amount: number | string
  created_at: string
  created_by?: string | null
}

export interface StagePayment {
  kind: string
  cost: number | string | null
  confirmed: boolean
  tranches?: StagePaymentTranche[]
}

export interface Stage {
  project_id: string
  parent_position: number
  position: number
  title: string
  deadline: string | null
  completed: boolean
  gip_confirmed: boolean
  payments: StagePayment[]
  has_act: boolean
}

export interface DetailedStage {
  project_id: string
  parent_position: number
  position: number
  title: string
  deadline: string | null
  completed: boolean
  payments: StagePayment[]
  gip_confirmed: boolean
}

export type Act = Attachment

export interface StageWithProjectTitle {
  stage_id: Stage
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
  is_pinned: boolean
}