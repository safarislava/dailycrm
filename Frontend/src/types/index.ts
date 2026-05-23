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

export interface StageWithProjectTitle {
  stage: Stage
  project_title: string
}