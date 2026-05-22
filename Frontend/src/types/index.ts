export interface Project {
  id: string
  title: string
}

export interface Stage {
  project_id: string
  position: number
  title: string
  deadline: string | null
}

export interface DetailedStage {
  stage: Stage
  description: string | null
  cost: number | null
}