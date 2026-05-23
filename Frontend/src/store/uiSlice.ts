import { createSlice, type PayloadAction } from '@reduxjs/toolkit'

interface UiState {
  selectedProjectId: string | null
  selectedStageId: string | null
  userPageOpen: boolean
}

const uiSlice = createSlice({
  name: 'ui',
  initialState: {
    selectedProjectId: null,
    selectedStageId: null,
    userPageOpen: false,
  } as UiState,
  reducers: {
    selectProject(state, action: PayloadAction<string | null>) {
      state.selectedProjectId = action.payload
      state.selectedStageId = null
    },
    selectStage(state, action: PayloadAction<string | null>) {
      state.selectedStageId = action.payload
    },
    setUserPageOpen(state, action: PayloadAction<boolean>) {
      state.userPageOpen = action.payload
    },
  },
})

export const { selectProject, selectStage, setUserPageOpen } = uiSlice.actions
export default uiSlice.reducer