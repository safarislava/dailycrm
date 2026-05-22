import { createSlice, type PayloadAction } from '@reduxjs/toolkit'

interface UiState {
  selectedProjectId: string | null
  selectedStageId: string | null
}

const uiSlice = createSlice({
  name: 'ui',
  initialState: {
    selectedProjectId: null,
    selectedStageId: null,
  } as UiState,
  reducers: {
    selectProject(state, action: PayloadAction<string | null>) {
      state.selectedProjectId = action.payload
      state.selectedStageId = null
    },
    selectStage(state, action: PayloadAction<string | null>) {
      state.selectedStageId = action.payload
    },
  },
})

export const { selectProject, selectStage } = uiSlice.actions
export default uiSlice.reducer