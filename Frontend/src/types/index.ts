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
  stage: Stage
  description: string | null
  cost: number | null
}

export interface DeadlineItem {
  project_id: string
  project_title: string
  position: number
  stage_title: string
  deadline: string
  completed: boolean
}