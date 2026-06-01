import { createSlice, type PayloadAction } from '@reduxjs/toolkit'

export type Theme = 'dark' | 'auto' | 'light'

interface UiState {
  selectedProjectId: string | null
  selectedStageId: string | null
  userPageOpen: boolean
  theme: Theme
}

const uiSlice = createSlice({
  name: 'ui',
  initialState: {
    selectedProjectId: null,
    selectedStageId: null,
    userPageOpen: false,
    theme: (localStorage.getItem('theme') as Theme | null) ?? 'auto',
  } as UiState,
  reducers: {
    selectProject(state, action: PayloadAction<string | null>) {
      state.selectedProjectId = action.payload
      state.selectedStageId = null
      state.userPageOpen = false
    },
    selectStage(state, action: PayloadAction<string | null>) {
      state.selectedStageId = action.payload
    },
    setUserPageOpen(state, action: PayloadAction<boolean>) {
      state.userPageOpen = action.payload
    },
    setTheme(state, action: PayloadAction<Theme>) {
      state.theme = action.payload
      localStorage.setItem('theme', action.payload)
    },
  },
})

export const { selectProject, selectStage, setUserPageOpen, setTheme } = uiSlice.actions
export default uiSlice.reducer