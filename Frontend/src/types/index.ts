export interface Project {
  id: string
  title: string
}

export interface Stage {
  project_id: string
  position: number
  title: string
}

export interface DetailedStage {
  stage: Stage
  description: string
  deadline: string
  cost: number
}