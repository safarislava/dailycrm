import {createSlice, type PayloadAction} from '@reduxjs/toolkit'

export type Theme = 'dark' | 'auto' | 'light'

export interface SelectedStage {
  parentPosition: number
  position: number
}

interface UiState {
  selectedProjectId: string | null
  selectedStage: SelectedStage | null
  userPageOpen: boolean
  adminPageOpen: boolean
  theme: Theme
  viewArchive: boolean
}

const uiSlice = createSlice({
  name: 'ui',
  initialState: {
    selectedProjectId: null,
    selectedStage: null,
    userPageOpen: false,
    adminPageOpen: false,
    theme: (localStorage.getItem('theme') as Theme | null) ?? 'auto',
    viewArchive: false,
  } as UiState,
  reducers: {
    setViewArchive(state, action: PayloadAction<boolean>) {
      if (state.viewArchive !== action.payload) {
        state.viewArchive = action.payload
        state.selectedProjectId = null
        state.selectedStage = null
      }
    },
    selectProject(state, action: PayloadAction<string | null>) {
      state.selectedProjectId = action.payload
      state.selectedStage = null
      state.userPageOpen = false
      state.adminPageOpen = false
    },
    selectStage(state, action: PayloadAction<SelectedStage | null>) {
      state.selectedStage = action.payload
    },
    setUserPageOpen(state, action: PayloadAction<boolean>) {
      state.userPageOpen = action.payload
      if (action.payload) {
        state.adminPageOpen = false
      }
    },
    setAdminPageOpen(state, action: PayloadAction<boolean>) {
      state.adminPageOpen = action.payload
      if (action.payload) {
        state.userPageOpen = false
        state.selectedProjectId = null
        state.selectedStage = null
      }
    },
    setTheme(state, action: PayloadAction<Theme>) {
      state.theme = action.payload
      localStorage.setItem('theme', action.payload)
    },
  },
})

export const {selectProject, selectStage, setUserPageOpen, setAdminPageOpen, setTheme, setViewArchive} = uiSlice.actions
export default uiSlice.reducer